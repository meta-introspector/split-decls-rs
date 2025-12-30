// Generated macro for is_jsonpath_safe (function)
macro_rules! Depcrate_json_findis_jsonpath_safe {
() => {
// Module: crate::json_find
// Provides: {"is_jsonpath_safe"}
// Dependencies: {}
fn is_jsonpath_safe (s : & str) -> bool { s . chars () . all (| c | c . is_ascii_alphanumeric () || c == '_') }
};
}
