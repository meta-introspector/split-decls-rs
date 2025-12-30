// Generated macro for impl_58 (impl)
macro_rules! Depcrate_task_wake_counterimpl_58 {
() => {
// Module: crate::task::wake_counter
// Provides: {"impl_58"}
// Dependencies: {}
impl AwokenCount { # [doc = " Get the current count."] pub fn get (& self) -> usize { self . inner . count . load (Ordering :: SeqCst) } }
};
}
