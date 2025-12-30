// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
# [cfg (feature = "crossbeam-channel")] impl DebounceEventHandler for crossbeam_channel :: Sender < DebounceEventResult > { fn handle_event (& mut self , event : DebounceEventResult) { let _ = self . send (event) ; } }
};
}
