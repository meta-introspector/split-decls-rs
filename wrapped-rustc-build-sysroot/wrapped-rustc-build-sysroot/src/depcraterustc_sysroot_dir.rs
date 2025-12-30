// Generated macro for rustc_sysroot_dir (function)
macro_rules! Depcraterustc_sysroot_dir {
() => {
// Module: crate
// Provides: {"rustc_sysroot_dir"}
// Dependencies: {}
fn rustc_sysroot_dir (mut rustc : Command) -> Result < PathBuf > { let output = rustc . args (["--print" , "sysroot"]) . output () . context ("failed to determine sysroot") ? ; if ! output . status . success () { bail ! ("failed to determine sysroot; rustc said:\n{}" , String :: from_utf8_lossy (& output . stderr) . trim_end ()) ; } let sysroot = std :: str :: from_utf8 (& output . stdout) . context ("sysroot folder is not valid UTF-8") ? ; let sysroot = PathBuf :: from (sysroot . trim_end_matches ('\n')) ; if ! sysroot . is_dir () { bail ! ("sysroot directory `{}` is not a directory" , sysroot . display ()) ; } Ok (sysroot) }
};
}
