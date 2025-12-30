// Generated macro for impl_654 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_654 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_654"}
// Dependencies: {}
impl Step for ErrorIndex { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/error_index_generator") . alias ("error-index") } fn make_run (run : RunConfig < '_ >) { let compilers = RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . builder . config . host_target ,) ; run . builder . ensure (ErrorIndex { compilers }) ; } # [doc = " Runs the error index generator tool to execute the tests located in the error"] # [doc = " index."] # [doc = ""] # [doc = " The `error_index_generator` tool lives in `src/tools` and is used to"] # [doc = " generate a markdown file from the error indexes of the code base which is"] # [doc = " then passed to `rustdoc --test`."] fn run (self , builder : & Builder < '_ >) { let target_compiler = self . compilers . target_compiler () ; let dir = testdir (builder , target_compiler . host) ; t ! (fs :: create_dir_all (& dir)) ; let output = dir . join ("error-index.md") ; let mut tool = tool :: ErrorIndex :: command (builder , self . compilers) ; tool . arg ("markdown") . arg (& output) ; let guard = builder . msg_test ("error-index" , target_compiler . host , target_compiler . stage) ; let _time = helpers :: timeit (builder) ; tool . run_capture (builder) ; drop (guard) ; builder . std (target_compiler , target_compiler . host) ; markdown_test (builder , target_compiler , & output) ; } }
};
}
