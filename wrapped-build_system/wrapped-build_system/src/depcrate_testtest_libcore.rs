// Generated macro for test_libcore (function)
macro_rules! Depcrate_testtest_libcore {
() => {
// Module: crate::test
// Provides: {"test_libcore"}
// Dependencies: {}
fn test_libcore (env : & Env , args : & TestArg) -> Result < () , String > { println ! ("[TEST] libcore") ; let path = get_sysroot_dir () . join ("sysroot_src/library/coretests") ; let _ = remove_dir_all (path . join ("target")) ; run_cargo_command (& [& "test"] , Some (& path) , env , args) ? ; Ok (()) }
};
}
