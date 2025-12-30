// Generated macro for impl_599 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_599 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_599"}
// Dependencies: {}
impl Miri { # [doc = " Run `cargo miri setup` for the given target, return where the Miri sysroot was put."] pub fn build_miri_sysroot (builder : & Builder < '_ > , compiler : Compiler , target : TargetSelection ,) -> PathBuf { let miri_sysroot = builder . out . join (compiler . host) . join ("miri-sysroot") ; let mut cargo = builder :: Cargo :: new (builder , compiler , Mode :: Std , SourceType :: Submodule , target , Kind :: MiriSetup ,) ; cargo . env ("MIRI_LIB_SRC" , builder . src . join ("library")) ; cargo . env ("MIRI_SYSROOT" , & miri_sysroot) ; let mut cargo = BootstrapCommand :: from (cargo) ; let _guard = builder . msg (Kind :: Build , "miri sysroot" , Mode :: ToolRustcPrivate , compiler , target) ; cargo . run (builder) ; cargo . arg ("--print-sysroot") ; builder . verbose (| | println ! ("running: {cargo:?}")) ; let stdout = cargo . run_capture_stdout (builder) . stdout () ; let sysroot = stdout . trim_end () ; builder . verbose (| | println ! ("`cargo miri setup --print-sysroot` said: {sysroot:?}")) ; PathBuf :: from (sysroot) } }
};
}
