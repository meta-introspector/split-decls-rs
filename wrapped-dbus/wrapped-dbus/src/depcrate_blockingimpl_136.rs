// Generated macro for impl_136 (impl)
macro_rules! Depcrate_blockingimpl_136 {
() => {
// Module: crate::blocking
// Provides: {"impl_136"}
// Dependencies: {}
impl SyncConnection { fn filters_mut (& self) -> std :: sync :: MutexGuard < Filters < SyncFilterCb > > { self . filters . lock () . unwrap () } }
};
}
