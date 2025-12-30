// Generated macro for impl_93 (impl)
macro_rules! Depcrate_rt_async_supportimpl_93 {
() => {
// Module: crate::rt::async_support
// Provides: {"impl_93"}
// Dependencies: {}
impl Drop for TaskCancelOnDrop { fn drop (& mut self) { # [cfg (not (target_arch = "wasm32"))] unsafe fn cancel () { unreachable ! () } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "[export]$root")] extern "C" { # [link_name = "[task-cancel]"] fn cancel () ; } unsafe { cancel () } } }
};
}
