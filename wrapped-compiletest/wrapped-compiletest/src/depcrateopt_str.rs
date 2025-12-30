// Generated macro for opt_str (function)
macro_rules! Depcrateopt_str {
() => {
// Module: crate
// Provides: {"opt_str"}
// Dependencies: {}
pub fn opt_str (maybestr : & Option < String >) -> & str { match * maybestr { None => "(none)" , Some (ref s) => s , } }
};
}
