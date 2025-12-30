// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
# [cfg (feature = "flume")] impl DebounceEventHandler for flume :: Sender < DebounceEventResult > { fn handle_event (& mut self , event : DebounceEventResult) { let _ = self . send (event) ; } }
};
}
