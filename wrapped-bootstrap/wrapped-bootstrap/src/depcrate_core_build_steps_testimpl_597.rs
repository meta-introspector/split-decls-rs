// Generated macro for impl_597 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_597 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_597"}
// Dependencies: {}
impl Step for Rustfmt { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/rustfmt") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Rustfmt { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . builder . host_target ,) , }) ; } # [doc = " Runs `cargo test` for rustfmt."] fn run (self , builder : & Builder < '_ >) { let tool_result = builder . ensure (tool :: Rustfmt :: from_compilers (self . compilers)) ; let build_compiler = tool_result . build_compiler ; let target = self . compilers . target () ; let mut cargo = tool :: prepare_tool_cargo (builder , build_compiler , Mode :: ToolRustcPrivate , target , Kind :: Test , "src/tools/rustfmt" , SourceType :: InTree , & [] ,) ; let dir = testdir (builder , target) ; t ! (fs :: create_dir_all (& dir)) ; cargo . env ("RUSTFMT_TEST_DIR" , dir) ; cargo . add_rustc_lib_path (builder) ; run_cargo_test (cargo , & [] , & [] , "rustfmt" , target , builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("rustfmt" , self . compilers . target ()) . built_by (self . compilers . build_compiler ()) ,) } }
};
}
