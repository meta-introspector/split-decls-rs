// Generated macro for MissingFields (struct)
macro_rules! Depcrate_diagnosticsMissingFields {
() => {
// Module: crate::diagnostics
// Provides: {"MissingFields"}
// Dependencies: {}
# [derive (Debug)] pub struct MissingFields { pub file : HirFileId , pub field_list_parent : AstPtr < Either < ast :: RecordExpr , ast :: RecordPat > > , pub field_list_parent_path : Option < AstPtr < ast :: Path > > , pub missed_fields : Vec < Name > , }
};
}
