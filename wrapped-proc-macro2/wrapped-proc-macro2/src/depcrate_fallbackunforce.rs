// Generated macro for unforce (function)
macro_rules! Depcrate_fallbackunforce {
() => {
// Module: crate::fallback
// Provides: {"unforce"}
// Dependencies: {}
# [doc = " Resume using the compiler's implementation of the proc macro API if it is"] # [doc = " available."] pub fn unforce () { # [cfg (wrap_proc_macro)] crate :: detection :: unforce_fallback () ; }
};
}
