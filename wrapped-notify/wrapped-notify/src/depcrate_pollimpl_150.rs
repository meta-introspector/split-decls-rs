// Generated macro for impl_150 (impl)
macro_rules! Depcrate_pollimpl_150 {
() => {
// Module: crate::poll
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (feature = "flume")] impl ScanEventHandler for flume :: Sender < ScanEvent > { fn handle_event (& mut self , event : ScanEvent) { let _ = self . send (event) ; } }
};
}
