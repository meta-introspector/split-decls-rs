// Generated macro for test (macro)
macro_rules! Depcrate_core_build_steps_testtest {
() => {
// Module: crate::core::build_steps::test
// Provides: {"test"}
// Dependencies: {}
# [doc = " Declares a test step that invokes compiletest on a particular test suite."] macro_rules ! test { ($ (# [$ attr : meta]) * $ name : ident { path : $ path : expr , mode : $ mode : expr , suite : $ suite : expr , default : $ default : expr $ (, IS_HOST : $ IS_HOST : expr) ? $ (, compare_mode : $ compare_mode : expr) ? $ (,) ? }) => { $ (# [$ attr]) * # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct $ name { test_compiler : Compiler , target : TargetSelection , } impl Step for $ name { type Output = () ; const DEFAULT : bool = $ default ; const IS_HOST : bool = (const { # [allow (unused_assignments , unused_mut)] let mut value = false ; $ (value = $ IS_HOST ;) ? value }) ; fn should_run (run : ShouldRun <'_ >) -> ShouldRun <'_ > { run . suite_path ($ path) } fn make_run (run : RunConfig <'_ >) { let test_compiler = run . builder . compiler (run . builder . top_stage , run . build_triple ()) ; run . builder . ensure ($ name { test_compiler , target : run . target }) ; } fn run (self , builder : & Builder <'_ >) { builder . ensure (Compiletest { test_compiler : self . test_compiler , target : self . target , mode : $ mode , suite : $ suite , path : $ path , compare_mode : (const { # [allow (unused_assignments , unused_mut)] let mut value = None ; $ (value = $ compare_mode ;) ? value }) , }) } } } ; }
};
}
