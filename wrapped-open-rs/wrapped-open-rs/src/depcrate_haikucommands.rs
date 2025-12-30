// Generated macro for commands (function)
macro_rules! Depcrate_haikucommands {
() => {
// Module: crate::haiku
// Provides: {"commands"}
// Dependencies: {}
pub fn commands < T : AsRef < OsStr > > (path : T) -> Vec < Command > { let mut cmd = Command :: new ("/bin/open") ; cmd . arg (path . as_ref ()) ; vec ! [cmd] }
};
}
