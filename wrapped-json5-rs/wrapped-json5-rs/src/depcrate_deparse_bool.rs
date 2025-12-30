// Generated macro for parse_bool (function)
macro_rules! Depcrate_deparse_bool {
() => {
// Module: crate::de
// Provides: {"parse_bool"}
// Dependencies: {}
fn parse_bool (pair : & Pair < '_ , Rule >) -> bool { match pair . as_str () { "true" => true , "false" => false , _ => unreachable ! () , } }
};
}
