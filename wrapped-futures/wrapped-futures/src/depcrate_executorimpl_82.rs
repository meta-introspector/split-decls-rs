// Generated macro for impl_82 (impl)
macro_rules! Depcrate_executorimpl_82 {
() => {
// Module: crate::executor
// Provides: {"impl_82"}
// Dependencies: {}
impl Executor for Inline { fn execute < F : FnOnce () + Send + 'static > (& self , f : F) { f () } fn execute_boxed (& self , f : Box < ExecuteCallback >) { f . call () } }
};
}
