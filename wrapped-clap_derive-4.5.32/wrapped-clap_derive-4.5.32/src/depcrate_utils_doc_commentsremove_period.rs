// Generated macro for remove_period (function)
macro_rules! Depcrate_utils_doc_commentsremove_period {
() => {
// Module: crate::utils::doc_comments
// Provides: {"remove_period"}
// Dependencies: {}
fn remove_period (mut s : String) -> String { if s . ends_with ('.') && ! s . ends_with ("..") { s . pop () ; } s }
};
}
