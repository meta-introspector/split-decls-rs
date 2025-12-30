// Generated macro for ScanEventHandler (trait)
macro_rules! Depcrate_pollScanEventHandler {
() => {
// Module: crate::poll
// Provides: {"ScanEventHandler"}
// Dependencies: {}
# [doc = " Handler trait for receivers of [`ScanEvent`]."] # [doc = " Very much the same as [`EventHandler`], but including the Result."] # [doc = ""] # [doc = " See the full example for more information."] pub trait ScanEventHandler : Send + 'static { # [doc = " Handles an event."] fn handle_event (& mut self , event : ScanEvent) ; }
};
}
