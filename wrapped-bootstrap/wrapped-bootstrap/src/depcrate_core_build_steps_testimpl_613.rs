// Generated macro for impl_613 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_613 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_613"}
// Dependencies: {}
impl Step for RustdocJSNotStd { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = run . builder . config . nodejs . is_some () ; run . suite_path ("tests/rustdoc-js") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { let compiler = run . builder . compiler (run . builder . top_stage , run . build_triple ()) ; run . builder . ensure (RustdocJSNotStd { target : run . target , compiler }) ; } fn run (self , builder : & Builder < '_ >) { builder . ensure (Compiletest { test_compiler : self . compiler , target : self . target , mode : "rustdoc-js" , suite : "rustdoc-js" , path : "tests/rustdoc-js" , compare_mode : None , }) ; } }
};
}
