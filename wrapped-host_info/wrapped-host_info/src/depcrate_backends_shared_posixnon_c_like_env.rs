// Generated macro for non_c_like_env (function)
macro_rules! Depcrate_backends_shared_posixnon_c_like_env {
() => {
// Module: crate::backends::shared::posix
// Provides: {"non_c_like_env"}
// Dependencies: {}
# [inline] fn non_c_like_env (name : & str) -> Option < String > { std :: env :: var_os (name) . and_then (| v | { let s = v . to_string_lossy () ; if s . is_empty () || is_c_like (& s) { None } else { Some (s . into_owned ()) } }) }
};
}
