// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl DebounceEventHandler for std :: sync :: mpsc :: Sender < DebounceEventResult > { fn handle_event (& mut self , event : DebounceEventResult) { let _ = self . send (event) ; } }
};
}
