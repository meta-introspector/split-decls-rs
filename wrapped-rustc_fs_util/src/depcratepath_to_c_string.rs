// Generated macro for path_to_c_string (function)
macro_rules! Depcratepath_to_c_string {
() => {
// Module: crate
// Provides: {"path_to_c_string"}
// Dependencies: {}
# [cfg (windows)] pub fn path_to_c_string (p : & Path) -> CString { CString :: new (p . to_str () . unwrap ()) . unwrap () }
};
}
