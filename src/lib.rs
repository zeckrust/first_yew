use serde::{Serialize, Deserialize};
use yew::prelude::*;
use gloo::console::log;

#[derive(Serialize, Deserialize)]


struct MyObject
{
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html
{
    let name: &str= "Gabriel";
    let my_object: MyObject = MyObject{username: name.to_owned(),
                                       favorite_language: "Rust".to_owned()};
    log!(name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let my_class: &str = "my_title";
    let message: Option<&str> = Some("I am a message");

    let tasks: Vec<&str> = vec!["record video",
                                 "grocery shopping",
                                 "eat",];

    html!
    {
        <>
            <h1 class={my_class}>{"Hello World!"}</h1>
            if my_class == "my_title"
            {
            <p>{"Hi there!"}</p>
            }
            else
            {
                <p>{"I am not a title"}</p>
            }

            if let Some(message) = message
            {
                <p>{message}</p>
            }
            else
            {
                <p>{"No messages to see today"}</p>
            }

            <ul>
                {list_to_html(tasks)}
            </ul>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html>
{
    list.iter().map(|item: &&str| html!{<li>{item}</li>}).collect()
}