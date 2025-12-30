// Generated macro for impl_609 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_609 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_609"}
// Dependencies: {}
impl Step for RustdocTheme { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/rustdoc-themes") } fn make_run (run : RunConfig < '_ >) { let test_compiler = run . builder . compiler (run . builder . top_stage , run . target) ; run . builder . ensure (RustdocTheme { test_compiler }) ; } fn run (self , builder : & Builder < '_ >) { let rustdoc = builder . bootstrap_out . join ("rustdoc") ; let mut cmd = builder . tool_cmd (Tool :: RustdocTheme) ; cmd . arg (rustdoc . to_str () . unwrap ()) . arg (builder . src . join ("src/librustdoc/html/static/css/rustdoc.css") . to_str () . unwrap ()) . env ("RUSTC_STAGE" , self . test_compiler . stage . to_string ()) . env ("RUSTC_SYSROOT" , builder . sysroot (self . test_compiler)) . env ("RUSTDOC_LIBDIR" , builder . sysroot_target_libdir (self . test_compiler , self . test_compiler . host) ,) . env ("CFG_RELEASE_CHANNEL" , & builder . config . channel) . env ("RUSTDOC_REAL" , builder . rustdoc_for_compiler (self . test_compiler)) . env ("RUSTC_BOOTSTRAP" , "1") ; cmd . args (linker_args (builder , self . test_compiler . host , LldThreads :: No)) ; cmd . delay_failure () . run (builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("rustdoc-theme" , self . test_compiler . host) . stage (self . test_compiler . stage) ,) } }
};
}
