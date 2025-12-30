// Generated macro for DebounceEventHandler (trait)
macro_rules! DepcrateDebounceEventHandler {
() => {
// Module: crate
// Provides: {"DebounceEventHandler"}
// Dependencies: {}
# [doc = " The set of requirements for watcher debounce event handling functions."] # [doc = ""] # [doc = " # Example implementation"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use notify::{Event, Result, EventHandler};"] # [doc = " # use notify_debouncer_full::{DebounceEventHandler, DebounceEventResult};"] # [doc = ""] # [doc = " /// Prints received events"] # [doc = " struct EventPrinter;"] # [doc = ""] # [doc = " impl DebounceEventHandler for EventPrinter {"] # [doc = "     fn handle_event(&mut self, result: DebounceEventResult) {"] # [doc = "         match result {"] # [doc = "             Ok(events) => events.iter().for_each(|event| println!(\"{event:?}\")),"] # [doc = "             Err(errors) => errors.iter().for_each(|error| println!(\"{error:?}\")),"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait DebounceEventHandler : Send + 'static { # [doc = " Handles an event."] fn handle_event (& mut self , event : DebounceEventResult) ; }
};
}
