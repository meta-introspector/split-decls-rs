// Generated macro for impl_13 (impl)
macro_rules! Depcrate_eitherimpl_13 {
() => {
// Module: crate::either
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "export")] impl < P0 , P1 > ExportableProvider for EitherProvider < P0 , P1 > where P0 : ExportableProvider , P1 : ExportableProvider , { fn supported_markers (& self) -> alloc :: collections :: BTreeSet < DataMarkerInfo > { use EitherProvider :: * ; match self { A (p) => p . supported_markers () , B (p) => p . supported_markers () , } } }
};
}
