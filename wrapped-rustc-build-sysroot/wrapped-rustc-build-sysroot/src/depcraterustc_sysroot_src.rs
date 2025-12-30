// Generated macro for rustc_sysroot_src (function)
macro_rules! Depcraterustc_sysroot_src {
() => {
// Module: crate
// Provides: {"rustc_sysroot_src"}
// Dependencies: {}
# [doc = " Returns where the given rustc stores its sysroot source code."] pub fn rustc_sysroot_src (rustc : Command) -> Result < PathBuf > { let sysroot = rustc_sysroot_dir (rustc) ? ; let rustc_src = sysroot . join ("lib") . join ("rustlib") . join ("src") . join ("rust") . join ("library") ; let rustc_src = rustc_src . canonicalize () . unwrap_or (rustc_src) ; Ok (rustc_src) }
};
}
