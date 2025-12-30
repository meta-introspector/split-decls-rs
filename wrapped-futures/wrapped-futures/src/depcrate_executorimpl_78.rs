// Generated macro for impl_78 (impl)
macro_rules! Depcrate_executorimpl_78 {
() => {
// Module: crate::executor
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : Executor + ? Sized + Send + Sync + 'static > Executor for Arc < T > { fn execute_boxed (& self , f : Box < ExecuteCallback >) { (* * self) . execute_boxed (f) } }
};
}
