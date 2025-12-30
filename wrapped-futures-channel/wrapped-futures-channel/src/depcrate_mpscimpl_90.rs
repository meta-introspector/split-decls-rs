// Generated macro for impl_90 (impl)
macro_rules! Depcrate_mpscimpl_90 {
() => {
// Module: crate::mpsc
// Provides: {"impl_90"}
// Dependencies: {}
impl SenderTask { fn new () -> Self { Self { task : None , is_parked : false } } fn notify (& mut self) { self . is_parked = false ; if let Some (task) = self . task . take () { task . wake () ; } } }
};
}
