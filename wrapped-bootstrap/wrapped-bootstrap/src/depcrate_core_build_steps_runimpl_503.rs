// Generated macro for impl_503 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_503 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_503"}
// Dependencies: {}
impl Step for CyclicStep { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("cyclic-step") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CyclicStep { n : 2 }) } fn run (self , builder : & Builder < '_ >) -> Self :: Output { builder . ensure (CyclicStep { n : self . n . saturating_sub (1) }) } }
};
}
