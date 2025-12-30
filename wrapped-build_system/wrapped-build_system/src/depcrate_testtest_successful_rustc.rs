// Generated macro for test_successful_rustc (function)
macro_rules! Depcrate_testtest_successful_rustc {
() => {
// Module: crate::test
// Provides: {"test_successful_rustc"}
// Dependencies: {}
fn test_successful_rustc (env : & Env , args : & TestArg) -> Result < () , String > { test_rustc_inner (env , args , remove_files_callback ("tests/failing-ui-tests.txt" , "ui") , false , "ui" ,) ? ; test_rustc_inner (env , args , remove_files_callback ("tests/failing-run-make-tests.txt" , "run-make") , false , "run-make" ,) ? ; test_rustc_inner (env , args , remove_files_callback ("tests/failing-run-make-tests.txt" , "run-make-cargo") , false , "run-make-cargo" ,) }
};
}
