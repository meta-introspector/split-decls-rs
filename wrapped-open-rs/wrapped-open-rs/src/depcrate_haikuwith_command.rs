// Generated macro for with_command (function)
macro_rules! Depcrate_haikuwith_command {
() => {
// Module: crate::haiku
// Provides: {"with_command"}
// Dependencies: {}
pub fn with_command < T : AsRef < OsStr > > (path : T , app : impl Into < String >) -> Command { let mut cmd = Command :: new (app . into ()) ; cmd . arg (path . as_ref ()) ; cmd }
};
}
