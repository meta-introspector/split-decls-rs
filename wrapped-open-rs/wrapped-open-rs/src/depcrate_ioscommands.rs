// Generated macro for commands (function)
macro_rules! Depcrate_ioscommands {
() => {
// Module: crate::ios
// Provides: {"commands"}
// Dependencies: {}
pub fn commands < T : AsRef < OsStr > > (path : T) -> Vec < Command > { let mut cmd = Command :: new ("uiopen") ; cmd . arg ("--url") . arg (path . as_ref ()) ; vec ! [cmd] }
};
}
