// Generated macro for try_restore (function)
macro_rules! Depcrate_inittry_restore {
() => {
// Module: crate::init
// Provides: {"try_restore"}
// Dependencies: {}
# [doc = " Restore the terminal to its original state."] # [doc = ""] # [doc = " This function will attempt to restore the terminal to its original state by performing the"] # [doc = " following steps:"] # [doc = ""] # [doc = " 1. Raw mode is disabled."] # [doc = " 2. The alternate screen buffer is left."] # [doc = ""] # [doc = " If either of these steps fail, the error is returned."] # [doc = ""] # [doc = " Use [`restore`] instead of this function when you don't need to handle the error yourself, as"] # [doc = " ignoring the error is generally the correct behavior when cleaning up before exiting. If you"] # [doc = " need to handle the error yourself, use this function instead."] # [doc = ""] # [doc = " See the [module-level documentation](mod@crate::init) for a comparison of all initialization"] # [doc = " functions and guidance on when to use each one."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " ratatui::try_restore()?;"] # [doc = " # Ok::<(), std::io::Error>(())"] # [doc = " ```"] pub fn try_restore () -> io :: Result < () > { disable_raw_mode () ? ; execute ! (stdout () , LeaveAlternateScreen) ? ; Ok (()) }
};
}
