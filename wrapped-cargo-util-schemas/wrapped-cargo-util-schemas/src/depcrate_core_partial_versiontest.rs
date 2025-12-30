// Generated macro for test (module)
macro_rules! Depcrate_core_partial_versiontest {
() => {
// Module: crate::core::partial_version
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use snapbox :: prelude :: * ; use snapbox :: str ; # [test] fn parse_success () { let cases = & [("1.43.0-beta.1" , str ! ["1.43.0-beta.1"]) , ("1.43.0-beta.1.x" , str ! ["1.43.0-beta.1.x"]) ,] ; for (input , expected) in cases { let actual : Result < PartialVersion , _ > = input . parse () ; let actual = match actual { Ok (result) => result . to_string () , Err (err) => format ! ("didn't pass: {err}") , } ; snapbox :: assert_data_eq ! (actual , expected . clone () . raw ()) ; } } # [test] fn parse_errors () { let cases = & [("^1.43" , str ! [[r#"unexpected version requirement, expected a version like "1.32""#]] ,) , ("1.43-beta.1" , str ! [[r#"unexpected prerelease field, expected a version like "1.32""#]] ,) , ("x" , str ! [[r#"unexpected version requirement, expected a version like "1.32""#]] ,) , ("1.x" , str ! [[r#"unexpected version requirement, expected a version like "1.32""#]] ,) , ("1.1.x" , str ! [[r#"unexpected version requirement, expected a version like "1.32""#]] ,) , ("foodaddle" , str ! [[r#"expected a version like "1.32""#]]) ,] ; for (input , expected) in cases { let actual : Result < PartialVersion , _ > = input . parse () ; let actual = match actual { Ok (result) => format ! ("didn't fail: {result:?}") , Err (err) => err . to_string () , } ; snapbox :: assert_data_eq ! (actual , expected . clone () . raw ()) ; } } }
};
}
