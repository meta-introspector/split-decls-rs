// Generated macro for context_get (function)
macro_rules! Depcrate_rt_async_supportcontext_get {
() => {
// Module: crate::rt::async_support
// Provides: {"context_get"}
// Dependencies: {}
fn context_get () -> * mut u8 { # [cfg (not (target_arch = "wasm32"))] unsafe fn get () -> * mut u8 { unreachable ! () } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "$root")] extern "C" { # [link_name = "[context-get-0]"] fn get () -> * mut u8 ; } unsafe { get () } }
};
}
