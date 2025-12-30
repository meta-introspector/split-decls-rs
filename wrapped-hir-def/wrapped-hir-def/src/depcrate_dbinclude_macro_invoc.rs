// Generated macro for include_macro_invoc (function)
macro_rules! Depcrate_dbinclude_macro_invoc {
() => {
// Module: crate::db
// Provides: {"include_macro_invoc"}
// Dependencies: {}
fn include_macro_invoc (db : & dyn DefDatabase , krate : Crate ,) -> Arc < [(MacroCallId , EditionedFileId)] > { crate_def_map (db , krate) . modules . values () . flat_map (| m | m . scope . iter_macro_invoc ()) . filter_map (| invoc | { db . lookup_intern_macro_call (* invoc . 1) . include_file_id (db , * invoc . 1) . map (| x | (* invoc . 1 , x)) }) . collect () }
};
}
