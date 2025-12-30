// Generated macro for restore (function)
macro_rules! Depcrate_initrestore {
() => {
// Module: crate::init
// Provides: {"restore"}
// Dependencies: {}
# [doc = " Restores the terminal to its original state."] # [doc = ""] # [doc = " This function should be called before the program exits to ensure that the terminal is"] # [doc = " restored to its original state."] # [doc = ""] # [doc = " This function will attempt to restore the terminal to its original state by performing the"] # [doc = " following steps:"] # [doc = ""] # [doc = " 1. Raw mode is disabled."] # [doc = " 2. The alternate screen buffer is left."] # [doc = ""] # [doc = " If either of these steps fail, the error is printed to stderr and ignored."] # [doc = ""] # [doc = " Use this function over [`try_restore`] when you don't need to handle the error yourself, as"] # [doc = " ignoring the error is generally the correct behavior when cleaning up before exiting. If you"] # [doc = " need to handle the error yourself, use [`try_restore`] instead."] # [doc = ""] # [doc = " See the [module-level documentation](mod@crate::init) for a comparison of all initialization"] # [doc = " functions and guidance on when to use each one."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " ratatui::restore();"] # [doc = " ```"] pub fn restore () { if let Err (err) = try_restore () { std :: eprintln ! ("Failed to restore terminal: {err}") ; } }
};
}
