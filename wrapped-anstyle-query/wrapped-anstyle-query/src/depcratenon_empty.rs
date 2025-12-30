// Generated macro for non_empty (function)
macro_rules! Depcratenon_empty {
() => {
// Module: crate
// Provides: {"non_empty"}
// Dependencies: {}
fn non_empty (var : Option < & std :: ffi :: OsStr >) -> bool { ! var . unwrap_or_default () . is_empty () }
};
}
