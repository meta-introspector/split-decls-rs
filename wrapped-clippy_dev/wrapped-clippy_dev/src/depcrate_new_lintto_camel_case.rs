// Generated macro for to_camel_case (function)
macro_rules! Depcrate_new_lintto_camel_case {
() => {
// Module: crate::new_lint
// Provides: {"to_camel_case"}
// Dependencies: {}
fn to_camel_case (name : & str) -> String { name . split ('_') . map (| s | { if s . is_empty () { String :: new () } else { [& s [0 .. 1] . to_uppercase () , & s [1 ..]] . concat () } }) . collect () }
};
}
