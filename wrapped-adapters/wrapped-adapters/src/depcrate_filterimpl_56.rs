// Generated macro for impl_56 (impl)
macro_rules! Depcrate_filterimpl_56 {
() => {
// Module: crate::filter
// Provides: {"impl_56"}
// Dependencies: {}
# [cfg (feature = "export")] impl < P0 , F > ExportableProvider for FilterDataProvider < P0 , F > where P0 : ExportableProvider , F : Fn (DataIdentifierBorrowed) -> bool + Sync , { fn supported_markers (& self) -> alloc :: collections :: BTreeSet < DataMarkerInfo > { self . inner . supported_markers () } }
};
}
