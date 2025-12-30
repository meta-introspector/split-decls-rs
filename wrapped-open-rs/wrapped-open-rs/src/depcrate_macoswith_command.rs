// Generated macro for with_command (function)
macro_rules! Depcrate_macoswith_command {
() => {
// Module: crate::macos
// Provides: {"with_command"}
// Dependencies: {}
pub fn with_command < T : AsRef < OsStr > > (path : T , app : impl Into < String >) -> Command { let mut cmd = Command :: new ("/usr/bin/open") ; cmd . arg (path . as_ref ()) . arg ("-a") . arg (app . into ()) ; cmd }
};
}
