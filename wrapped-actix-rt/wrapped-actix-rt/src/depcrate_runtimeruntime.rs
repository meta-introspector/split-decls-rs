// Generated macro for Runtime (struct)
macro_rules! Depcrate_runtimeRuntime {
() => {
// Module: crate::runtime
// Provides: {"Runtime"}
// Dependencies: {}
# [doc = " A Tokio-based runtime proxy."] # [doc = ""] # [doc = " All spawned futures will be executed on the current thread. Therefore, there is no `Send` bound"] # [doc = " on submitted futures."] # [derive (Debug)] pub struct Runtime { local : LocalSet , rt : tokio :: runtime :: Runtime , }
};
}
