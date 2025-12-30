// Generated macro for DebounceEventHandler (trait)
macro_rules! DepcrateDebounceEventHandler {
() => {
// Module: crate
// Provides: {"DebounceEventHandler"}
// Dependencies: {}
# [doc = " The set of requirements for watcher debounce event handling functions."] # [doc = ""] # [doc = " # Example implementation"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use notify::{Event, Result, EventHandler};"] # [doc = " # use notify_debouncer_mini::{DebounceEventHandler,DebounceEventResult};"] # [doc = ""] # [doc = " /// Prints received events"] # [doc = " struct EventPrinter;"] # [doc = ""] # [doc = " impl DebounceEventHandler for EventPrinter {"] # [doc = "     fn handle_event(&mut self, event: DebounceEventResult) {"] # [doc = "         match event {"] # [doc = "             Ok(events) => {"] # [doc = "                 for event in events {"] # [doc = "                     println!(\"Event {:?} for path {:?}\",event.kind,event.path);"] # [doc = "                 }"] # [doc = "             },"] # [doc = "             // errors are immediately reported"] # [doc = "             Err(error) => println!(\"Got error {:?}\",error),"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait DebounceEventHandler : Send + 'static { # [doc = " Handles an event."] fn handle_event (& mut self , event : DebounceEventResult) ; }
};
}
