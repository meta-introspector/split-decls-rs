// Generated macro for impl_79 (impl)
macro_rules! Depcrate_fork_by_errorimpl_79 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "export")] impl < P0 , P1 , F > ExportableProvider for ForkByErrorProvider < P0 , P1 , F > where P0 : ExportableProvider , P1 : ExportableProvider , F : ForkByErrorPredicate + Sync , { fn supported_markers (& self) -> alloc :: collections :: BTreeSet < DataMarkerInfo > { let mut markers = self . 0 . supported_markers () ; markers . extend (self . 1 . supported_markers ()) ; markers } }
};
}
