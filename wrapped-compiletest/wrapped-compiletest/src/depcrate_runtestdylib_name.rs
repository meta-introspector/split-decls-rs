// Generated macro for dylib_name (function)
macro_rules! Depcrate_runtestdylib_name {
() => {
// Module: crate::runtest
// Provides: {"dylib_name"}
// Dependencies: {}
fn dylib_name (name : & str) -> String { format ! ("{}{name}.{}" , std :: env :: consts :: DLL_PREFIX , std :: env :: consts :: DLL_EXTENSION) }
};
}
