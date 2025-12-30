// Generated macro for read (function)
macro_rules! Depcrate_eventread {
() => {
// Module: crate::event
// Provides: {"read"}
// Dependencies: {}
# [doc = " Reads a single [`Event`](enum.Event.html)."] # [doc = ""] # [doc = " This function blocks until an [`Event`](enum.Event.html) is available. Combine it with the"] # [doc = " [`poll`](fn.poll.html) function to get non-blocking reads."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Blocking read:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use crossterm::event::read;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " fn print_events() -> io::Result<bool> {"] # [doc = "     loop {"] # [doc = "         // Blocks until an `Event` is available"] # [doc = "         println!(\"{:?}\", read()?);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Non-blocking read:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::time::Duration;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " use crossterm::event::{read, poll};"] # [doc = ""] # [doc = " fn print_events() -> io::Result<bool> {"] # [doc = "     loop {"] # [doc = "         if poll(Duration::from_millis(100))? {"] # [doc = "             // It's guaranteed that `read` won't block, because `poll` returned"] # [doc = "             // `Ok(true)`."] # [doc = "             println!(\"{:?}\", read()?);"] # [doc = "         } else {"] # [doc = "             // Timeout expired, no `Event` is available"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn read () -> std :: io :: Result < Event > { match internal :: read (& EventFilter) ? { InternalEvent :: Event (event) => Ok (event) , # [cfg (unix)] _ => unreachable ! () , } }
};
}
