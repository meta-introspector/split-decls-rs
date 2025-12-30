// Generated macro for backpressure_inc (function)
macro_rules! Depcrate_rt_async_supportbackpressure_inc {
() => {
// Module: crate::rt::async_support
// Provides: {"backpressure_inc"}
// Dependencies: {}
# [doc = " Call the `backpressure.inc` canonical built-in function."] pub fn backpressure_inc () { # [cfg (not (target_arch = "wasm32"))] unsafe fn backpressure_inc () { unreachable ! () ; } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "$root")] extern "C" { # [link_name = "[backpressure-inc]"] fn backpressure_inc () ; } unsafe { backpressure_inc () } }
};
}
