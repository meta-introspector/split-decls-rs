// Generated macro for Logger (struct)
macro_rules! Depcrate_ffiLogger {
() => {
// Module: crate::ffi
// Provides: {"Logger"}
// Dependencies: {}
struct Logger { cb : extern "C" fn (line : * const u8 , argp : * mut c_void) , argp : atomic :: AtomicPtr < c_void > , }
};
}
