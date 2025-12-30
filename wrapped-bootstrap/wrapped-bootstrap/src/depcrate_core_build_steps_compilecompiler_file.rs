// Generated macro for compiler_file (function)
macro_rules! Depcrate_core_build_steps_compilecompiler_file {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"compiler_file"}
// Dependencies: {}
pub fn compiler_file (builder : & Builder < '_ > , compiler : & Path , target : TargetSelection , c : CLang , file : & str ,) -> PathBuf { if builder . config . dry_run () { return PathBuf :: new () ; } let mut cmd = command (compiler) ; cmd . args (builder . cc_handled_clags (target , c)) ; cmd . args (builder . cc_unhandled_cflags (target , GitRepo :: Rustc , c)) ; cmd . arg (format ! ("-print-file-name={file}")) ; let out = cmd . run_capture_stdout (builder) . stdout () ; PathBuf :: from (out . trim ()) }
};
}
