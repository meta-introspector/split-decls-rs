// Generated macro for catch_unwind (function)
macro_rules! Depcrate_taskcatch_unwind {
() => {
// Module: crate::task
// Provides: {"catch_unwind"}
// Dependencies: {}
fn catch_unwind < F , U > (f : F) -> thread :: Result < U > where F : FnOnce () -> U + Send + 'static , { panic :: catch_unwind (panic :: AssertUnwindSafe (f)) }
};
}
