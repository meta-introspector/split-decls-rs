// Generated macro for impl_149 (impl)
macro_rules! Depcrate_os_windowsimpl_149 {
() => {
// Module: crate::os::windows
// Provides: {"impl_149"}
// Dependencies: {}
impl ErrorModeGuard { # [allow (clippy :: if_same_then_else)] fn new () -> Option < ErrorModeGuard > { unsafe { let mut previous_mode = 0 ; if SetThreadErrorMode (SEM_FAILCRITICALERRORS , & mut previous_mode) == 0 { None } else if previous_mode == SEM_FAILCRITICALERRORS { None } else { Some (ErrorModeGuard (previous_mode)) } } } }
};
}
