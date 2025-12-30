// Generated macro for poll (function)
macro_rules! Depcrate_eventpoll {
() => {
// Module: crate::event
// Provides: {"poll"}
// Dependencies: {}
# [doc = " Checks if there is an [`Event`](enum.Event.html) available."] # [doc = ""] # [doc = " Returns `Ok(true)` if an [`Event`](enum.Event.html) is available otherwise it returns `Ok(false)`."] # [doc = ""] # [doc = " `Ok(true)` guarantees that subsequent call to the [`read`](fn.read.html) function"] # [doc = " won't block."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `timeout` - maximum waiting time for event availability"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Return immediately:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::{time::Duration, io};"] # [doc = " use crossterm::{event::poll};"] # [doc = ""] # [doc = " fn is_event_available() -> io::Result<bool> {"] # [doc = "     // Zero duration says that the `poll` function must return immediately"] # [doc = "     // with an `Event` availability information"] # [doc = "     poll(Duration::from_secs(0))"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Wait up to 100ms:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::{time::Duration, io};"] # [doc = ""] # [doc = " use crossterm::event::poll;"] # [doc = ""] # [doc = " fn is_event_available() -> io::Result<bool> {"] # [doc = "     // Wait for an `Event` availability for 100ms. It returns immediately"] # [doc = "     // if an `Event` is/becomes available."] # [doc = "     poll(Duration::from_millis(100))"] # [doc = " }"] # [doc = " ```"] pub fn poll (timeout : Duration) -> std :: io :: Result < bool > { internal :: poll (Some (timeout) , & EventFilter) }
};
}
