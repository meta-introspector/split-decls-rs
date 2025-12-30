// Generated macro for SyncObj (trait)
macro_rules! Depcrate_concurrency_syncSyncObj {
() => {
// Module: crate::concurrency::sync
// Provides: {"SyncObj"}
// Dependencies: {}
# [doc = " A trait for the synchronization metadata that can be attached to a memory location."] pub trait SyncObj : Any { # [doc = " Determines whether reads/writes to this object's location are currently permitted."] fn on_access < 'tcx > (& self , _access_kind : AccessKind) -> InterpResult < 'tcx > { interp_ok (()) } # [doc = " Determines whether this object's metadata shall be deleted when a write to its"] # [doc = " location occurs."] fn delete_on_write (& self) -> bool { false } }
};
}
