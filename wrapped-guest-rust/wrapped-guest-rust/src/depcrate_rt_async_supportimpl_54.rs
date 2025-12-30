// Generated macro for impl_54 (impl)
macro_rules! Depcrate_rt_async_supportimpl_54 {
() => {
// Module: crate::rt::async_support
// Provides: {"impl_54"}
// Dependencies: {}
impl Drop for FutureState < '_ > { fn drop (& mut self) { if ! self . tasks . is_empty () { self . with_p3_task_set (| me | { me . tasks = Default :: default () ; }) } } }
};
}
