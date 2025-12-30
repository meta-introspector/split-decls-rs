// Generated macro for match_acceptable_sym (function)
macro_rules! Depcrate_manual_retainmatch_acceptable_sym {
() => {
// Module: crate::manual_retain
// Provides: {"match_acceptable_sym"}
// Dependencies: {}
fn match_acceptable_sym (cx : & LateContext < '_ > , collect_def_id : DefId) -> bool { cx . tcx . get_diagnostic_name (collect_def_id) . is_some_and (| collect_name | ACCEPTABLE_METHODS . contains (& collect_name)) }
};
}
