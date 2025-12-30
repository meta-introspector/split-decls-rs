// Generated macro for MainThreadBound (struct)
macro_rules! Depcrate_main_thread_boundMainThreadBound {
() => {
// Module: crate::main_thread_bound
// Provides: {"MainThreadBound"}
// Dependencies: {}
# [doc = " Make a type that can only be used on the main thread be `Send` + `Sync`."] # [doc = ""] # [doc = " On `Drop`, the inner type is sent to the main thread's runloop and dropped"] # [doc = " there. This may lead to deadlocks if the main runloop is not running, or"] # [doc = " if it is waiting on a lock that the dropping thread is holding. See"] # [doc = " [`run_on_main`] for some of the caveats around that."] # [doc = ""] # [doc = ""] # [doc = " # Related"] # [doc = ""] # [doc = " This type takes inspiration from `threadbound::ThreadBound`."] # [doc = ""] # [doc = " The functionality also somewhat resembles Swift's `@MainActor`, which"] # [doc = " ensures that a type is only usable from the main thread."] # [doc (alias = "@MainActor")] pub struct MainThreadBound < T > (ManuallyDrop < T >) ;
};
}
