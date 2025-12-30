// Generated macro for impl_1537 (impl)
macro_rules! Depcrate_shims_unix_macos_syncimpl_1537 {
() => {
// Module: crate::shims::unix::macos::sync
// Provides: {"impl_1537"}
// Dependencies: {}
impl SyncObj for MacOsUnfairLock { fn on_access < 'tcx > (& self , access_kind : AccessKind) -> InterpResult < 'tcx > { if let MacOsUnfairLock :: Active { mutex_ref } = self && ! mutex_ref . queue_is_empty () { throw_ub_format ! ("{access_kind} of `os_unfair_lock` is forbidden while the queue is non-empty") ; } interp_ok (()) } fn delete_on_write (& self) -> bool { true } }
};
}
