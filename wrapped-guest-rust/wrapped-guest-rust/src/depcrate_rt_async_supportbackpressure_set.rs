// Generated macro for backpressure_set (function)
macro_rules! Depcrate_rt_async_supportbackpressure_set {
() => {
// Module: crate::rt::async_support
// Provides: {"backpressure_set"}
// Dependencies: {}
# [doc = " Call the `backpressure.set` canonical built-in function."] # [doc = ""] # [doc = " When `enabled` is `true`, this tells the host to defer any new calls to this"] # [doc = " component instance until further notice (i.e. until `backpressure.set` is"] # [doc = " called again with `enabled` set to `false`)."] # [deprecated = "use backpressure_{inc,dec} instead"] pub fn backpressure_set (enabled : bool) { # [cfg (not (target_arch = "wasm32"))] unsafe fn backpressure_set (_ : i32) { unreachable ! () ; } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "$root")] extern "C" { # [link_name = "[backpressure-set]"] fn backpressure_set (_ : i32) ; } unsafe { backpressure_set (if enabled { 1 } else { 0 }) } }
};
}
