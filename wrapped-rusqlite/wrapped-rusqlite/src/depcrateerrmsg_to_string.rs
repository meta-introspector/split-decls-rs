// Generated macro for errmsg_to_string (function)
macro_rules! Depcrateerrmsg_to_string {
() => {
// Module: crate
// Provides: {"errmsg_to_string"}
// Dependencies: {}
unsafe fn errmsg_to_string (errmsg : * const c_char) -> String { CStr :: from_ptr (errmsg) . to_string_lossy () . into_owned () }
};
}
