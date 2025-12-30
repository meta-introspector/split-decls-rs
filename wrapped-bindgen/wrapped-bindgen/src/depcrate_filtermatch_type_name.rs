// Generated macro for match_type_name (function)
macro_rules! Depcrate_filtermatch_type_name {
() => {
// Module: crate::filter
// Provides: {"match_type_name"}
// Dependencies: {}
fn match_type_name (rule : & str , namespace : & str , name : & str) -> bool { if rule . len () <= namespace . len () { return namespace . starts_with (rule) ; } if ! rule . starts_with (namespace) { return false ; } if rule . as_bytes () [namespace . len ()] != b'.' { return false ; } name == & rule [namespace . len () + 1 ..] }
};
}
