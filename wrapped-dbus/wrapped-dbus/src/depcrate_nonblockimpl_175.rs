// Generated macro for impl_175 (impl)
macro_rules! Depcrate_nonblockimpl_175 {
() => {
// Module: crate::nonblock
// Provides: {"impl_175"}
// Dependencies: {}
impl SyncConnection { fn filters_mut (& self) -> std :: sync :: MutexGuard < Filters < SyncFilterCb > > { self . filters . lock () . unwrap () } fn replies_mut (& self) -> std :: sync :: MutexGuard < Replies < SyncRepliesCb > > { self . replies . lock () . unwrap () } }
};
}
