// Generated macro for to_initial_case (function)
macro_rules! Depcrate_admonitionsto_initial_case {
() => {
// Module: crate::admonitions
// Provides: {"to_initial_case"}
// Dependencies: {}
fn to_initial_case (s : & str) -> String { let mut chars = s . chars () ; let first = chars . next () . expect ("not empty") . to_uppercase () ; let rest = chars . as_str () . to_lowercase () ; format ! ("{first}{rest}") }
};
}
