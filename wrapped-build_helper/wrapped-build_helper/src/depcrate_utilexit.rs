// Generated macro for exit (macro)
macro_rules! Depcrate_utilexit {
() => {
// Module: crate::util
// Provides: {"exit"}
// Dependencies: {}
# [doc = " Invokes `build_helper::util::detail_exit` with `cfg!(test)`"] # [doc = ""] # [doc = " This is a macro instead of a function so that it uses `cfg(test)` in the *calling* crate, not in build helper."] # [macro_export] macro_rules ! exit { ($ code : expr) => { $ crate :: util :: detail_exit ($ code , cfg ! (test)) ; } ; }
};
}
