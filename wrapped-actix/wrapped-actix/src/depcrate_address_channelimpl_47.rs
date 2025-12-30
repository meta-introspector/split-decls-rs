// Generated macro for impl_47 (impl)
macro_rules! Depcrate_address_channelimpl_47 {
() => {
// Module: crate::address::channel
// Provides: {"impl_47"}
// Dependencies: {}
impl SenderTask { fn new () -> Self { SenderTask { task : None , is_parked : false , } } fn notify (& mut self) -> bool { self . is_parked = false ; if let Some (task) = self . task . take () { task . wake () ; true } else { false } } }
};
}
