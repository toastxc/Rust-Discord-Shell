extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use std::process::Command;

// discord bot token
const TOKEN : &str = "";



struct Handler;

impl EventHandler for Handler {

    fn message(&self, ctx: Context, msg: Message) {


        // discord user ID

        let toastusr: u64 = ;

        // bot ID

        let _botusr: u64 = ;

        if  msg.content.chars().next().unwrap() == '?' {
        

            let msg_content = &msg.content.split_at(1);

            let msg_content = msg_content.1;

            println!("{}", msg_content);

        if msg.author.id == toastusr {
            

            // help command (put what you want)

        if msg_content == "help" { 
            if let Err(why1) = msg.channel_id.say(&ctx.http, "**DivanSH** `prefix = ?`") {
                println!("Error giving message: {:?}", why1);
            }
        }
    

            else {
                
                println!("started!");
                let usr_input = msg_content;

                let strlen = usr_input.split(" ");

                let mut collect = vec!["init"];
                let mut index = 0;

                for s in strlen {

                    collect.push("num");
                    index = index + 1;
                    collect[index] = s;
                };

                let mut com = Command::new(collect[1]);



                println!("{}", index);
                    for x in 1..index {
                        com.arg(collect[x+1]);
                        println!("??{}", collect[x]);
                    } 

                    let stdout = com.output().unwrap().stdout;
                    let stderr = com.output().unwrap().stderr;

                    let printable_stdout = String::from_utf8_lossy(&stdout);
                    let printable_stderr = String::from_utf8_lossy(&stderr);

                    println!("output: {}\nerror: {}", printable_stdout, printable_stderr);

               
                    let finprint = "```\n".to_owned() + &printable_stdout.to_string() + "```\n";
                    msg.channel_id.say(&ctx.http, finprint);
                    msg.channel_id.say(&ctx.http, printable_stderr);
            } 
                }
        }
    }
    
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

    fn main() {
        let mut client = Client::new(&TOKEN, Handler)
                            .expect("Error creating client");


        if let Err(msg) = client.start() {
            println!("Error: {:?}", msg);
        }


    }
    

