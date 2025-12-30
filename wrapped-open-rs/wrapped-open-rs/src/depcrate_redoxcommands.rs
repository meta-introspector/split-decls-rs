// Generated macro for commands (function)
macro_rules! Depcrate_redoxcommands {
() => {
// Module: crate::redox
// Provides: {"commands"}
// Dependencies: {}
pub fn commands < T : AsRef < OsStr > > (path : T) -> Vec < Command > { let mut cmd = Command :: new ("launcher") ; cmd . arg (path . as_ref ()) ; vec ! [cmd] }
};
}
