// Generated macro for impl_649 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_649 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_649"}
// Dependencies: {}
impl Step for BookTest { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) { if self . is_ext_doc { self . run_ext_doc (builder) ; } else { self . run_local_doc (builder) ; } } }
};
}
