// Generated macro for to_c_str (function)
macro_rules! Depcrateto_c_str {
() => {
// Module: crate
// Provides: {"to_c_str"}
// Dependencies: {}
fn to_c_str (n : & str) -> CString { CString :: new (n . as_bytes ()) . unwrap () }
};
}
