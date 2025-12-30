// Generated macro for impl_595 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_595 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_595"}
// Dependencies: {}
impl Step for RustAnalyzer { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/rust-analyzer") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Self { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . builder . host_target ,) , }) ; } # [doc = " Runs `cargo test` for rust-analyzer"] fn run (self , builder : & Builder < '_ >) { let host = self . compilers . target () ; let workspace_path = "src/tools/rust-analyzer" ; let crate_path = "src/tools/rust-analyzer/crates/proc-macro-srv" ; let mut cargo = tool :: prepare_tool_cargo (builder , self . compilers . build_compiler () , Mode :: ToolRustcPrivate , host , Kind :: Test , crate_path , SourceType :: InTree , & ["in-rust-tree" . to_owned ()] ,) ; cargo . allow_features (tool :: RustAnalyzer :: ALLOW_FEATURES) ; let dir = builder . src . join (workspace_path) ; cargo . env ("CARGO_WORKSPACE_DIR" , & dir) ; cargo . env ("SKIP_SLOW_TESTS" , "1") ; cargo . add_rustc_lib_path (builder) ; run_cargo_test (cargo , & [] , & [] , "rust-analyzer" , host , builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("rust-analyzer" , self . compilers . target ()) . built_by (self . compilers . build_compiler ()) ,) } }
};
}
