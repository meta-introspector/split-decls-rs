// Generated macro for to_header_name (function)
macro_rules! Depcrateto_header_name {
() => {
// Module: crate
// Provides: {"to_header_name"}
// Dependencies: {}
fn to_header_name (ty_name : & str) -> String { let mut out = String :: new () ; let mut first = true ; for c in ty_name . chars () { if first { out . push (c . to_ascii_uppercase ()) ; first = false ; } else { if c . is_uppercase () { out . push ('_') ; } out . push (c . to_ascii_uppercase ()) ; } } out }
};
}
