// Generated macro for context_set (function)
macro_rules! Depcrate_rt_async_supportcontext_set {
() => {
// Module: crate::rt::async_support
// Provides: {"context_set"}
// Dependencies: {}
unsafe fn context_set (value : * mut u8) { # [cfg (not (target_arch = "wasm32"))] unsafe fn set (_ : * mut u8) { unreachable ! () } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "$root")] extern "C" { # [link_name = "[context-set-0]"] fn set (value : * mut u8) ; } unsafe { set (value) } }
};
}
