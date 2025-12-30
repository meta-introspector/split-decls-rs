// Generated macro for build_commandline (function)
macro_rules! Depcrate_processbuild_commandline {
() => {
// Module: crate::process
// Provides: {"build_commandline"}
// Dependencies: {}
fn build_commandline (command : & Command) -> OsString { let mut buf = OsString :: new () ; buf . push (command . get_program ()) ; for arg in command . get_args () { buf . push (" ") ; buf . push (arg) ; } buf }
};
}
