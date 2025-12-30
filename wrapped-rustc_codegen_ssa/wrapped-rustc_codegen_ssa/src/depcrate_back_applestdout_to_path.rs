// Generated macro for stdout_to_path (function)
macro_rules! Depcrate_back_applestdout_to_path {
() => {
// Module: crate::back::apple
// Provides: {"stdout_to_path"}
// Dependencies: {}
fn stdout_to_path (mut stdout : Vec < u8 >) -> PathBuf { if let Some (b'\n') = stdout . last () { let _ = stdout . pop () . unwrap () ; } # [cfg (unix)] let path = < OsString as std :: os :: unix :: ffi :: OsStringExt > :: from_vec (stdout) ; # [cfg (not (unix))] let path = OsString :: from (String :: from_utf8 (stdout) . expect ("stdout must be UTF-8")) ; PathBuf :: from (path) }
};
}
