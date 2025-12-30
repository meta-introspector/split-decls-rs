// Generated macro for build_sysroot (function)
macro_rules! Depcrate_testbuild_sysroot {
() => {
// Module: crate::test
// Provides: {"build_sysroot"}
// Dependencies: {}
fn build_sysroot (env : & Env , args : & TestArg) -> Result < () , String > { println ! ("[BUILD] sysroot") ; let mut config = args . config_info . clone () ; config . features . extend (args . sysroot_features . iter () . cloned ()) ; build :: build_sysroot (env , & config) ? ; Ok (()) }
};
}
