// Generated macro for dynamic_library (function)
macro_rules! Depcrate_generateddynamic_library {
() => {
// Module: crate::generated
// Provides: {"dynamic_library"}
// Dependencies: {}
pub (crate) fn dynamic_library () -> & 'static libloading :: Library { let path = match std :: env :: var ("DYNAMIC_LIBZ_SYS") { Ok (path) => path , Err (e) => panic ! ("could not read env var DYNAMIC_LIBZ_SYS: {e}") , } ; use std :: sync :: OnceLock ; static LIB : OnceLock < libloading :: Library > = OnceLock :: new () ; LIB . get_or_init (| | unsafe { libloading :: Library :: new (path) } . unwrap ()) }
};
}
