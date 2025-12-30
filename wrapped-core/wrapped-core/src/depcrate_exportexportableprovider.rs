// Generated macro for ExportableProvider (trait)
macro_rules! Depcrate_exportExportableProvider {
() => {
// Module: crate::export
// Provides: {"ExportableProvider"}
// Dependencies: {}
# [doc = " A [`DynamicDataProvider`] that can be used for exporting data."] # [doc = ""] # [doc = " Use [`make_exportable_provider`] to implement this."] pub trait ExportableProvider : crate :: data_provider :: IterableDynamicDataProvider < ExportMarker > + Sync { # [doc = " Returns the set of supported markers"] fn supported_markers (& self) -> BTreeSet < DataMarkerInfo > ; }
};
}
