// Generated macro for impl_663 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_663 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_663"}
// Dependencies: {}
impl Step for CrateRustdoc { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . paths (& ["src/librustdoc" , "src/tools/rustdoc"]) } fn make_run (run : RunConfig < '_ >) { let builder = run . builder ; builder . ensure (CrateRustdoc { host : run . target }) ; } fn run (self , builder : & Builder < '_ >) { let target = self . host ; let compiler = if builder . download_rustc () { builder . compiler (builder . top_stage , target) } else { builder . compiler_for (builder . top_stage , target , target) } ; builder . std (compiler , target) ; builder . ensure (compile :: Rustc :: new (compiler , target)) ; let mut cargo = tool :: prepare_tool_cargo (builder , compiler , Mode :: ToolRustcPrivate , target , builder . kind , "src/tools/rustdoc" , SourceType :: InTree , & [] ,) ; if self . host . contains ("musl") { cargo . arg ("'-Ctarget-feature=-crt-static'") ; } let libdir = if builder . download_rustc () { builder . rustc_libdir (compiler) } else { builder . sysroot_target_libdir (compiler , target) . to_path_buf () } ; let mut dylib_path = dylib_path () ; dylib_path . insert (0 , PathBuf :: from (& * libdir)) ; cargo . env (dylib_path_var () , env :: join_paths (& dylib_path) . unwrap ()) ; run_cargo_test (cargo , & [] , & ["rustdoc:0.0.0" . to_string ()] , "rustdoc" , target , builder) ; } }
};
}
