// Generated macro for with_command (function)
macro_rules! Depcrate_ioswith_command {
() => {
// Module: crate::ios
// Provides: {"with_command"}
// Dependencies: {}
pub fn with_command < T : AsRef < OsStr > > (path : T , app : impl Into < String >) -> Command { let mut cmd = Command :: new ("uiopen") ; cmd . arg ("--url") . arg (path . as_ref ()) . arg ("--bundleid") . arg (app . into ()) ; cmd }
};
}
