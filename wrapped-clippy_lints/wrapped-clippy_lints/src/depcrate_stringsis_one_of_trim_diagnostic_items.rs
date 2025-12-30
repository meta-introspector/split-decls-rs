// Generated macro for is_one_of_trim_diagnostic_items (function)
macro_rules! Depcrate_stringsis_one_of_trim_diagnostic_items {
() => {
// Module: crate::strings
// Provides: {"is_one_of_trim_diagnostic_items"}
// Dependencies: {}
fn is_one_of_trim_diagnostic_items (cx : & LateContext < '_ > , trim_def_id : DefId) -> bool { matches ! (cx . tcx . get_diagnostic_name (trim_def_id) , Some (sym :: str_trim | sym :: str_trim_start | sym :: str_trim_end)) }
};
}
