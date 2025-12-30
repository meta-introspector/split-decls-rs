// Generated macro for rust_backtrace_full (function)
macro_rules! Depcrate_panic_hookrust_backtrace_full {
() => {
// Module: crate::panic_hook
// Provides: {"rust_backtrace_full"}
// Dependencies: {}
fn rust_backtrace_full () -> bool { static RUST_BACKTRACE_FULL : LazyLock < bool > = LazyLock :: new (| | matches ! (env :: var ("RUST_BACKTRACE") . as_deref () , Ok ("full"))) ; * RUST_BACKTRACE_FULL }
};
}
