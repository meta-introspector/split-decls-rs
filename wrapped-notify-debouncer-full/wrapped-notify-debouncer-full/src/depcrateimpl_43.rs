// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg (feature = "crossbeam-channel")] impl DebounceEventHandler for crossbeam_channel :: Sender < DebounceEventResult > { fn handle_event (& mut self , event : DebounceEventResult) { let _ = self . send (event) ; } }
};
}
