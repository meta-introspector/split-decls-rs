// Generated macro for path_to_cstring (function)
macro_rules! Depcratepath_to_cstring {
() => {
// Module: crate
// Provides: {"path_to_cstring"}
// Dependencies: {}
# [cfg (not (unix))] fn path_to_cstring (p : & Path) -> Result < CString > { let s = p . to_str () . ok_or_else (| | Error :: InvalidPath (p . to_owned ())) ? ; Ok (CString :: new (s) ?) }
};
}
