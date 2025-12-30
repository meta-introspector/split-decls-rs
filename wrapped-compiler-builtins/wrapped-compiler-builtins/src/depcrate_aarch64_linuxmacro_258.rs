// Generated macro for macro_258 (macro)
macro_rules! Depcrate_aarch64_linuxmacro_258 {
() => {
// Module: crate::aarch64_linux
// Provides: {"macro_258"}
// Dependencies: {}
intrinsics ! { # [doc = " Call to enable LSE in outline atomic operations. The caller must verify"] # [doc = " LSE operations are supported."] pub extern "C" fn __rust_enable_lse () { HAVE_LSE_ATOMICS . store (1 , Ordering :: Relaxed) ; } }
};
}
