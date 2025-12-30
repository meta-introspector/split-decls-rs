// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_shims_unix_syncimpl_1344 {
() => {
// Module: crate::shims::unix::sync
// Provides: {"impl_1344"}
// Dependencies: {}
impl SyncObj for PthreadRwLock { fn on_access < 'tcx > (& self , access_kind : AccessKind) -> InterpResult < 'tcx > { if ! self . rwlock_ref . queue_is_empty () { throw_ub_format ! ("{access_kind} of `pthread_rwlock_t` is forbidden while the queue is non-empty") ; } interp_ok (()) } fn delete_on_write (& self) -> bool { true } }
};
}
