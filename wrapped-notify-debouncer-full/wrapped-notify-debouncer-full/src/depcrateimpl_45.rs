// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl DebounceEventHandler for std :: sync :: mpsc :: Sender < DebounceEventResult > { fn handle_event (& mut self , event : DebounceEventResult) { let _ = self . send (event) ; } }
};
}
