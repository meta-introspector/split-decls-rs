// Generated macro for impl_148 (impl)
macro_rules! Depcrate_pollimpl_148 {
() => {
// Module: crate::poll
// Provides: {"impl_148"}
// Dependencies: {}
impl < F > ScanEventHandler for F where F : FnMut (ScanEvent) + Send + 'static , { fn handle_event (& mut self , event : ScanEvent) { (self) (event) ; } }
};
}
