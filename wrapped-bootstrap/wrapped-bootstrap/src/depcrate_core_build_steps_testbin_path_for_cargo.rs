// Generated macro for bin_path_for_cargo (function)
macro_rules! Depcrate_core_build_steps_testbin_path_for_cargo {
() => {
// Module: crate::core::build_steps::test
// Provides: {"bin_path_for_cargo"}
// Dependencies: {}
fn bin_path_for_cargo (builder : & Builder < '_ > , compiler : Compiler) -> OsString { let path = builder . sysroot (compiler) . join ("bin") ; let old_path = env :: var_os ("PATH") . unwrap_or_default () ; env :: join_paths (iter :: once (path) . chain (env :: split_paths (& old_path))) . expect ("") }
};
}
