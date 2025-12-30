// Generated macro for OpaqueTypeStorage (struct)
macro_rules! Depcrate_infer_opaque_types_tableOpaqueTypeStorage {
() => {
// Module: crate::infer::opaque_types::table
// Provides: {"OpaqueTypeStorage"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] pub struct OpaqueTypeStorage < 'tcx > { opaque_types : FxIndexMap < OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx > > , duplicate_entries : Vec < (OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >) > , }
};
}
