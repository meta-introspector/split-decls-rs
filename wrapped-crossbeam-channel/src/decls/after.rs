macro_rules! deps {
    () => {
        Receiver!();
        ReceiverFlavor!();
        Channel!();
    };
}

macro_rules! after {
    () => {
        deps!();
        # [doc = " Creates a receiver that delivers a message after a certain duration of time."] # [doc = ""] # [doc = " The channel is bounded with capacity of 1 and never gets disconnected. Exactly one message will"] # [doc = " be sent into the channel after `duration` elapses. The message is the instant at which it is"] # [doc = " sent."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Using an `after` channel for timeouts:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::{after, select, unbounded};"] # [doc = ""] # [doc = " let (s, r) = unbounded::<i32>();"] # [doc = " let timeout = Duration::from_millis(100);"] # [doc = ""] # [doc = " select! {"] # [doc = "     recv(r) -> msg => println!(\"received {:?}\", msg),"] # [doc = "     recv(after(timeout)) -> _ => println!(\"timed out\"),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " When the message gets sent:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::{Duration, Instant};"] # [doc = " use crossbeam_channel::after;"] # [doc = ""] # [doc = " // Converts a number of milliseconds into a `Duration`."] # [doc = " let ms = |ms| Duration::from_millis(ms);"] # [doc = ""] # [doc = " // Returns `true` if `a` and `b` are very close `Instant`s."] # [doc = " let eq = |a, b| a + ms(60) > b && b + ms(60) > a;"] # [doc = ""] # [doc = " let start = Instant::now();"] # [doc = " let r = after(ms(100));"] # [doc = ""] # [doc = " thread::sleep(ms(500));"] # [doc = ""] # [doc = " // This message was sent 100 ms from the start and received 500 ms from the start."] # [doc = " assert!(eq(r.recv().unwrap(), start + ms(100)));"] # [doc = " assert!(eq(Instant::now(), start + ms(500)));"] # [doc = " ```"] pub fn after (duration : Duration) -> Receiver < Instant > { match Instant :: now () . checked_add (duration) { Some (deadline) => Receiver { flavor : ReceiverFlavor :: At (Arc :: new (flavors :: at :: Channel :: new_deadline (deadline))) , } , None => never () , } }
    };
}

after!()