// Generated macro for impl_149 (impl)
macro_rules! Depcrate_pollimpl_149 {
() => {
// Module: crate::poll
// Provides: {"impl_149"}
// Dependencies: {}
# [cfg (feature = "crossbeam-channel")] impl ScanEventHandler for crossbeam_channel :: Sender < ScanEvent > { fn handle_event (& mut self , event : ScanEvent) { let _ = self . send (event) ; } }
};
}
