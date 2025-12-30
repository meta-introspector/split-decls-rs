// Generated macro for force (function)
macro_rules! Depcrate_fallbackforce {
() => {
// Module: crate::fallback
// Provides: {"force"}
// Dependencies: {}
# [doc = " Force use of proc-macro2's fallback implementation of the API for now, even"] # [doc = " if the compiler's implementation is available."] pub fn force () { # [cfg (wrap_proc_macro)] crate :: detection :: force_fallback () ; }
};
}
