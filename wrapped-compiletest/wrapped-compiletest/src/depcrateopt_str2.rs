// Generated macro for opt_str2 (function)
macro_rules! Depcrateopt_str2 {
() => {
// Module: crate
// Provides: {"opt_str2"}
// Dependencies: {}
pub fn opt_str2 (maybestr : Option < String >) -> String { match maybestr { None => "(none)" . to_owned () , Some (s) => s , } }
};
}
