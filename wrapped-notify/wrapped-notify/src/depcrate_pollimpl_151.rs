// Generated macro for impl_151 (impl)
macro_rules! Depcrate_pollimpl_151 {
() => {
// Module: crate::poll
// Provides: {"impl_151"}
// Dependencies: {}
impl ScanEventHandler for std :: sync :: mpsc :: Sender < ScanEvent > { fn handle_event (& mut self , event : ScanEvent) { let _ = self . send (event) ; } }
};
}
