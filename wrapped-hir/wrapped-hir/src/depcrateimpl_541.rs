// Generated macro for impl_541 (impl)
macro_rules! Depcrateimpl_541 {
() => {
// Module: crate
// Provides: {"impl_541"}
// Dependencies: {}
impl < F > MethodCandidateCallback for F where F : FnMut (Function) -> ControlFlow < () > , { fn on_inherent_method (& mut self , f : Function) -> ControlFlow < () > { self (f) } fn on_trait_method (& mut self , f : Function) -> ControlFlow < () > { self (f) } }
};
}
