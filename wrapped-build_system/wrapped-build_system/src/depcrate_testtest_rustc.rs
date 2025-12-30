// Generated macro for test_rustc (function)
macro_rules! Depcrate_testtest_rustc {
() => {
// Module: crate::test
// Provides: {"test_rustc"}
// Dependencies: {}
fn test_rustc (env : & Env , args : & TestArg) -> Result < () , String > { test_rustc_inner (env , args , | _ | Ok (false) , false , "run-make") ? ; test_rustc_inner (env , args , | _ | Ok (false) , false , "run-make-cargo") ? ; test_rustc_inner (env , args , | _ | Ok (false) , false , "ui") }
};
}
