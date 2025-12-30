// Generated macro for impl_679 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_679 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_679"}
// Dependencies: {}
impl Step for LintDocs { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let stage = run . builder . top_stage ; run . path ("src/tools/lint-docs") . default_condition (stage > 1) } fn make_run (run : RunConfig < '_ >) { if run . builder . top_stage < 2 { eprintln ! ("WARNING: lint-docs tests might not work below stage 2") ; } run . builder . ensure (LintDocs { build_compiler : prepare_doc_compiler (run . builder , run . builder . config . host_target , run . builder . top_stage ,) , target : run . target , }) ; } # [doc = " Tests that the lint examples in the rustc book generate the correct"] # [doc = " lints and have the expected format."] fn run (self , builder : & Builder < '_ >) { builder . ensure (crate :: core :: build_steps :: doc :: RustcBook :: validate (self . build_compiler , self . target ,)) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("lint-docs" , self . target) . built_by (self . build_compiler)) } }
};
}
