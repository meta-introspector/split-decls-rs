// Generated macro for platform_defaults_to_one_thread (function)
macro_rules! Depcrateplatform_defaults_to_one_thread {
() => {
// Module: crate
// Provides: {"platform_defaults_to_one_thread"}
// Dependencies: {}
# [doc = " Returns whether the current host platform should use a single thread by"] # [doc = " default rather than a thread pool by default. Some platforms, such as"] # [doc = " WebAssembly, don't have native support for threading at this time."] fn platform_defaults_to_one_thread () -> bool { cfg ! (target_family = "wasm") }
};
}
