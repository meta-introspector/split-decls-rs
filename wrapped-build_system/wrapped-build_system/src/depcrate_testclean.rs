// Generated macro for clean (function)
macro_rules! Depcrate_testclean {
() => {
// Module: crate::test
// Provides: {"clean"}
// Dependencies: {}
fn clean (_env : & Env , args : & TestArg) -> Result < () , String > { let _ = remove_dir_all (& args . config_info . cargo_target_dir) ; let path = Path :: new (& args . config_info . cargo_target_dir) . join ("gccjit") ; create_dir (& path) }
};
}
