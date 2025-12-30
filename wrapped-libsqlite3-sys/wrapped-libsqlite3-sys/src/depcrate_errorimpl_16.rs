// Generated macro for impl_16 (impl)
macro_rules! Depcrate_errorimpl_16 {
() => {
// Module: crate::error
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (feature = "loadable_extension")] impl fmt :: Display for InitError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: VersionMismatch { compile_time , runtime , } => { write ! (f , "SQLite version mismatch: {runtime} < {compile_time}") } Self :: NullFunctionPointer => { write ! (f , "Some sqlite3_api_routines fields are null") } } } }
};
}
