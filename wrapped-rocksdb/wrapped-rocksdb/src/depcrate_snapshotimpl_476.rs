// Generated macro for impl_476 (impl)
macro_rules! Depcrate_snapshotimpl_476 {
() => {
// Module: crate::snapshot
// Provides: {"impl_476"}
// Dependencies: {}
impl < D : DBAccess > Drop for SnapshotWithThreadMode < '_ , D > { fn drop (& mut self) { unsafe { self . db . release_snapshot (self . inner) ; } } }
};
}
