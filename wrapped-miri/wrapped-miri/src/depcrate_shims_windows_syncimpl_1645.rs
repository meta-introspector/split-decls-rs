// Generated macro for impl_1645 (impl)
macro_rules! Depcrate_shims_windows_syncimpl_1645 {
() => {
// Module: crate::shims::windows::sync
// Provides: {"impl_1645"}
// Dependencies: {}
impl SyncObj for WindowsInitOnce { fn on_access < 'tcx > (& self , access_kind : AccessKind) -> InterpResult < 'tcx > { if ! self . init_once . queue_is_empty () { throw_ub_format ! ("{access_kind} of `INIT_ONCE` is forbidden while the queue is non-empty") ; } interp_ok (()) } fn delete_on_write (& self) -> bool { true } }
};
}
