// Generated macro for never (function)
macro_rules! Depcrate_channelnever {
() => {
// Module: crate::channel
// Provides: {"never"}
// Dependencies: {}
# [doc = " Creates a receiver that never delivers messages."] # [doc = ""] # [doc = " The channel is bounded with capacity of 0 and never gets disconnected."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Using a `never` channel to optionally add a timeout to [`select!`]:"] # [doc = ""] # [doc = " [`select!`]: crate::select!"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::{after, select, never, unbounded};"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = ""] # [doc = " # let t ="] # [doc = " thread::spawn(move || {"] # [doc = "     thread::sleep(Duration::from_secs(1));"] # [doc = "     s.send(1).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " // Suppose this duration can be a `Some` or a `None`."] # [doc = " let duration = Some(Duration::from_millis(100));"] # [doc = ""] # [doc = " // Create a channel that times out after the specified duration."] # [doc = " let timeout = duration"] # [doc = "     .map(after)"] # [doc = "     .unwrap_or_else(never);"] # [doc = ""] # [doc = " select! {"] # [doc = "     recv(r) -> msg => assert_eq!(msg, Ok(1)),"] # [doc = "     recv(timeout) -> _ => println!(\"timed out\"),"] # [doc = " }"] # [doc = " # t.join().unwrap(); // join thread to avoid https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] pub fn never < T > () -> Receiver < T > { Receiver { flavor : ReceiverFlavor :: Never (flavors :: never :: Channel :: new ()) , } }
};
}
