// Generated macro for test_failing_rustc (function)
macro_rules! Depcrate_testtest_failing_rustc {
() => {
// Module: crate::test
// Provides: {"test_failing_rustc"}
// Dependencies: {}
fn test_failing_rustc (env : & Env , args : & TestArg) -> Result < () , String > { let run_make_result = test_rustc_inner (env , args , retain_files_callback ("tests/failing-run-make-tests.txt" , "run-make") , false , "run-make" ,) ; let run_make_cargo_result = test_rustc_inner (env , args , retain_files_callback ("tests/failing-run-make-tests.txt" , "run-make-cargo") , false , "run-make" ,) ; let ui_result = test_rustc_inner (env , args , retain_files_callback ("tests/failing-ui-tests.txt" , "ui") , false , "ui" ,) ; run_make_result . and (run_make_cargo_result) . and (ui_result) }
};
}
