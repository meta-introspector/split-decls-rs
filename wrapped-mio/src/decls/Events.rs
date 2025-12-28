macro_rules! deps {
    () => {
        Poll!();
    };
}

macro_rules! Events {
    () => {
        deps!();
        # [doc = " A collection of readiness events."] # [doc = ""] # [doc = " `Events` is passed as an argument to [`Poll::poll`] and will be used to"] # [doc = " receive any new readiness events received since the last poll. Usually, a"] # [doc = " single `Events` instance is created at the same time as a [`Poll`] and"] # [doc = " reused on each call to [`Poll::poll`]."] # [doc = ""] # [doc = " See [`Poll`] for more documentation on polling."] # [doc = ""] # [doc = " [`Poll::poll`]: ../struct.Poll.html#method.poll"] # [doc = " [`Poll`]: ../struct.Poll.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [cfg_attr (feature = "os-poll" , doc = "```")] # [cfg_attr (not (feature = "os-poll") , doc = "```ignore")] # [doc = " # use std::error::Error;"] # [doc = " # fn main() -> Result<(), Box<dyn Error>> {"] # [doc = " use mio::{Events, Poll};"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let mut events = Events::with_capacity(1024);"] # [doc = " let mut poll = Poll::new()?;"] # [doc = " #"] # [doc = " # assert!(events.is_empty());"] # [doc = ""] # [doc = " // Register `event::Source`s with `poll`."] # [doc = ""] # [doc = " poll.poll(&mut events, Some(Duration::from_millis(100)))?;"] # [doc = ""] # [doc = " for event in events.iter() {"] # [doc = "     println!(\"Got an event for {:?}\", event.token());"] # [doc = " }"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] pub struct Events { inner : sys :: Events , }
    };
}

Events!()