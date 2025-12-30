// Generated macro for SyncUnsafeCell (struct)
macro_rules! Depcrate___macros_sync_unsafe_cellSyncUnsafeCell {
() => {
// Module: crate::__macros::sync_unsafe_cell
// Provides: {"SyncUnsafeCell"}
// Dependencies: {}
# [doc = " `UnsafeCell`, but `Sync`."] # [repr (transparent)] # [derive (Debug)] pub struct SyncUnsafeCell < T : ? Sized > { value : UnsafeCell < T > , }
};
}
