// Generated macro for test_book (macro)
macro_rules! Depcrate_core_build_steps_testtest_book {
() => {
// Module: crate::core::build_steps::test
// Provides: {"test_book"}
// Dependencies: {}
macro_rules ! test_book { ($ ($ name : ident , $ path : expr , $ book_name : expr , default =$ default : expr $ (, submodules = $ submodules : expr) ? $ (, dependencies =$ dependencies : expr) ? ;) +) => { $ (# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct $ name { test_compiler : Compiler , } impl Step for $ name { type Output = () ; const DEFAULT : bool = $ default ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun <'_ >) -> ShouldRun <'_ > { run . path ($ path) } fn make_run (run : RunConfig <'_ >) { run . builder . ensure ($ name { test_compiler : run . builder . compiler (run . builder . top_stage , run . target) , }) ; } fn run (self , builder : & Builder <'_ >) { $ (for submodule in $ submodules { builder . require_submodule (submodule , None) ; }) * let dependencies = vec ! [] ; $ (let mut dependencies = dependencies ; for dep in $ dependencies { dependencies . push (dep) ; }) ? builder . ensure (BookTest { test_compiler : self . test_compiler , path : PathBuf :: from ($ path) , name : $ book_name , is_ext_doc : !$ default , dependencies , }) ; } }) + } }
};
}
