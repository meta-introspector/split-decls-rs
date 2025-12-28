macro_rules! deps {
    () => {
        Poll!();
        Events!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " [`Events`] iterator."] # [doc = ""] # [doc = " This struct is created by the [`iter`] method on [`Events`]."] # [doc = ""] # [doc = " [`Events`]: struct.Events.html"] # [doc = " [`iter`]: struct.Events.html#method.iter"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [cfg_attr (feature = "os-poll" , doc = "```")] # [cfg_attr (not (feature = "os-poll") , doc = "```ignore")] # [doc = " # use std::error::Error;"] # [doc = " # fn main() -> Result<(), Box<dyn Error>> {"] # [doc = " use mio::{Events, Poll};"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let mut events = Events::with_capacity(1024);"] # [doc = " let mut poll = Poll::new()?;"] # [doc = ""] # [doc = " // Register handles with `poll`."] # [doc = ""] # [doc = " poll.poll(&mut events, Some(Duration::from_millis(100)))?;"] # [doc = ""] # [doc = " for event in events.iter() {"] # [doc = "     println!(\"Got an event for {:?}\", event.token());"] # [doc = " }"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Iter < 'a > { inner : & 'a Events , pos : usize , }
    };
}

Iter!();