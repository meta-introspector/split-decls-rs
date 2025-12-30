// Generated macro for impl_80 (impl)
macro_rules! Depcrate_executorimpl_80 {
() => {
// Module: crate::executor
// Provides: {"impl_80"}
// Dependencies: {}
impl < F : FnOnce () + Send + 'static > ExecuteCallback for F { fn call (self : Box < F >) { (* self) () } }
};
}
