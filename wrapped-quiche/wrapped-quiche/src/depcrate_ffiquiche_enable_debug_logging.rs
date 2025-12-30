// Generated macro for quiche_enable_debug_logging (function)
macro_rules! Depcrate_ffiquiche_enable_debug_logging {
() => {
// Module: crate::ffi
// Provides: {"quiche_enable_debug_logging"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_enable_debug_logging (cb : extern "C" fn (line : * const u8 , argp : * mut c_void) , argp : * mut c_void ,) -> c_int { let argp = atomic :: AtomicPtr :: new (argp) ; let logger = Box :: new (Logger { cb , argp }) ; if log :: set_boxed_logger (logger) . is_err () { return - 1 ; } log :: set_max_level (log :: LevelFilter :: Trace) ; 0 }
};
}
