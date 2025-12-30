// Generated macro for ExitGuard (struct)
macro_rules! Depcrate_oomExitGuard {
() => {
// Module: crate::oom
// Provides: {"ExitGuard"}
// Dependencies: {}
pub (crate) struct ExitGuard < F : FnOnce () > { drop_callback : Option < F > , }
};
}
