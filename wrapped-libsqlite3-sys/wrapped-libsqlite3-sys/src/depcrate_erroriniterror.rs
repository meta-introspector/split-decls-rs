// Generated macro for InitError (enum)
macro_rules! Depcrate_errorInitError {
() => {
// Module: crate::error
// Provides: {"InitError"}
// Dependencies: {}
# [doc = " Loadable extension initialization error"] # [cfg (feature = "loadable_extension")] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum InitError { # [doc = " Version mismatch between the extension and the SQLite3 library"] VersionMismatch { compile_time : i32 , runtime : i32 } , # [doc = " Invalid function pointer in one of `sqlite3_api_routines` fields"] NullFunctionPointer , }
};
}
