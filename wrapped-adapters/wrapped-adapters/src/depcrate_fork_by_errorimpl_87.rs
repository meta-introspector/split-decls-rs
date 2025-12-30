// Generated macro for impl_87 (impl)
macro_rules! Depcrate_fork_by_errorimpl_87 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (feature = "export")] impl < P , F > ExportableProvider for MultiForkByErrorProvider < P , F > where P : ExportableProvider , F : ForkByErrorPredicate + Sync , { fn supported_markers (& self) -> alloc :: collections :: BTreeSet < DataMarkerInfo > { self . providers . iter () . flat_map (| p | p . supported_markers ()) . collect () } }
};
}
