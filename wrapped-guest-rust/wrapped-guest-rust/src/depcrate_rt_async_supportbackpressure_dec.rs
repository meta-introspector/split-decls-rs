// Generated macro for backpressure_dec (function)
macro_rules! Depcrate_rt_async_supportbackpressure_dec {
() => {
// Module: crate::rt::async_support
// Provides: {"backpressure_dec"}
// Dependencies: {}
# [doc = " Call the `backpressure.dec` canonical built-in function."] pub fn backpressure_dec () { # [cfg (not (target_arch = "wasm32"))] unsafe fn backpressure_dec () { unreachable ! () ; } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "$root")] extern "C" { # [link_name = "[backpressure-dec]"] fn backpressure_dec () ; } unsafe { backpressure_dec () } }
};
}
