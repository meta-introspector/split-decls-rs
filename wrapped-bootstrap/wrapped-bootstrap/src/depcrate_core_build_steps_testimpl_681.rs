// Generated macro for impl_681 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_681 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_681"}
// Dependencies: {}
impl Step for RustInstaller { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; # [doc = " Ensure the version placeholder replacement tool builds"] fn run (self , builder : & Builder < '_ >) { let bootstrap_host = builder . config . host_target ; let build_compiler = builder . compiler (0 , bootstrap_host) ; let cargo = tool :: prepare_tool_cargo (builder , build_compiler , Mode :: ToolBootstrap , bootstrap_host , Kind :: Test , "src/tools/rust-installer" , SourceType :: InTree , & [] ,) ; let _guard = builder . msg_test ("rust-installer" , bootstrap_host , 1) ; run_cargo_test (cargo , & [] , & [] , None , bootstrap_host , builder) ; if bootstrap_host != "x86_64-unknown-linux-gnu" { return ; } let mut cmd = command (builder . src . join ("src/tools/rust-installer/test.sh")) ; let tmpdir = testdir (builder , build_compiler . host) . join ("rust-installer") ; let _ = std :: fs :: remove_dir_all (& tmpdir) ; let _ = std :: fs :: create_dir_all (& tmpdir) ; cmd . current_dir (& tmpdir) ; cmd . env ("CARGO_TARGET_DIR" , tmpdir . join ("cargo-target")) ; cmd . env ("CARGO" , & builder . initial_cargo) ; cmd . env ("RUSTC" , & builder . initial_rustc) ; cmd . env ("TMP_DIR" , & tmpdir) ; cmd . delay_failure () . run (builder) ; } fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/rust-installer") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Self) ; } }
};
}
