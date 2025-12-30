// Generated macro for discover_sysroot_dir (function)
macro_rules! Depcrate_sysrootdiscover_sysroot_dir {
() => {
// Module: crate::sysroot
// Provides: {"discover_sysroot_dir"}
// Dependencies: {}
fn discover_sysroot_dir (current_dir : & AbsPath , extra_env : & FxHashMap < String , Option < String > > ,) -> Result < AbsPathBuf > { let mut rustc = toolchain :: command (Tool :: Rustc . path () , current_dir , extra_env) ; rustc . current_dir (current_dir) . args (["--print" , "sysroot"]) ; tracing :: debug ! ("Discovering sysroot by {:?}" , rustc) ; let stdout = utf8_stdout (& mut rustc) ? ; Ok (AbsPathBuf :: assert (Utf8PathBuf :: from (stdout))) }
};
}
