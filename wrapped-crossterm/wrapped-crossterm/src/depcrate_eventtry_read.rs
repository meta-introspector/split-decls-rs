// Generated macro for try_read (function)
macro_rules! Depcrate_eventtry_read {
() => {
// Module: crate::event
// Provides: {"try_read"}
// Dependencies: {}
# [doc = " Attempts to read a single [`Event`](enum.Event.html) without blocking the thread."] # [doc = ""] # [doc = " If no event is found, `None` is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use crossterm::event::{try_read, poll};"] # [doc = " use std::{io, time::Duration};"] # [doc = ""] # [doc = " fn print_all_events() -> io::Result<bool> {"] # [doc = "     loop {"] # [doc = "         if poll(Duration::from_millis(100))? {"] # [doc = "             // Fetch *all* available events at once"] # [doc = "             while let Some(event) = try_read() {"] # [doc = "                 // ..."] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn try_read () -> Option < Event > { match internal :: try_read (& EventFilter) { Some (InternalEvent :: Event (event)) => Some (event) , None => None , # [cfg (unix)] _ => unreachable ! () , } }
};
}
