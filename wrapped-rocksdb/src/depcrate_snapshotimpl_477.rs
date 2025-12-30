// Generated macro for impl_477 (impl)
macro_rules! Depcrate_snapshotimpl_477 {
() => {
// Module: crate::snapshot
// Provides: {"impl_477"}
// Dependencies: {}
# [doc = " `Send` and `Sync` implementations for `SnapshotWithThreadMode` are safe, because `SnapshotWithThreadMode` is"] # [doc = " immutable and can be safely shared between threads."] unsafe impl < D : DBAccess > Send for SnapshotWithThreadMode < '_ , D > { }
};
}
