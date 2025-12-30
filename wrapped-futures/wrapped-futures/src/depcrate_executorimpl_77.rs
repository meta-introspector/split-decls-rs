// Generated macro for impl_77 (impl)
macro_rules! Depcrate_executorimpl_77 {
() => {
// Module: crate::executor
// Provides: {"impl_77"}
// Dependencies: {}
impl < T : Executor + ? Sized + Send + Sync + 'static > Executor for Box < T > { fn execute_boxed (& self , f : Box < ExecuteCallback >) { (* * self) . execute_boxed (f) } }
};
}
