// Generated macro for is_format_macro (function)
macro_rules! Depcrate_macrosis_format_macro {
() => {
// Module: crate::macros
// Provides: {"is_format_macro"}
// Dependencies: {}
# [doc = " Returns true if a given Macro `DefId` is a format macro (e.g. `println!`)"] pub fn is_format_macro (cx : & LateContext < '_ > , macro_def_id : DefId) -> bool { if let Some (name) = cx . tcx . get_diagnostic_name (macro_def_id) { FORMAT_MACRO_DIAG_ITEMS . contains (& name) } else { get_unique_builtin_attr (cx . sess () , cx . tcx . get_all_attrs (macro_def_id) , sym :: format_args) . is_some () } }
};
}
