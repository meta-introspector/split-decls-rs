// Generated macro for EventHandler (trait)
macro_rules! DepcrateEventHandler {
() => {
// Module: crate
// Provides: {"EventHandler"}
// Dependencies: {}
# [doc = " The set of requirements for watcher event handling functions."] # [doc = ""] # [doc = " # Example implementation"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use notify::{Event, Result, EventHandler};"] # [doc = ""] # [doc = " /// Prints received events"] # [doc = " struct EventPrinter;"] # [doc = ""] # [doc = " impl EventHandler for EventPrinter {"] # [doc = "     fn handle_event(&mut self, event: Result<Event>) {"] # [doc = "         if let Ok(event) = event {"] # [doc = "             println!(\"Event: {:?}\", event);"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait EventHandler : Send + 'static { # [doc = " Handles an event."] fn handle_event (& mut self , event : Result < Event >) ; }
};
}
