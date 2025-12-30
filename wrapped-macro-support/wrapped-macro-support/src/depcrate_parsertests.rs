// Generated macro for tests (module)
macro_rules! Depcrate_parsertests {
() => {
// Module: crate::parser
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_try_unescape () { use super :: try_unescape ; assert_eq ! (try_unescape ("hello") . unwrap () , "hello") ; assert_eq ! (try_unescape ("\"hello") . unwrap () , "hello") ; assert_eq ! (try_unescape ("hello\"") . unwrap () , "hello") ; assert_eq ! (try_unescape ("\"hello\"") . unwrap () , "hello") ; assert_eq ! (try_unescape ("hello\\\\") . unwrap () , "hello\\") ; assert_eq ! (try_unescape ("hello\\n") . unwrap () , "hello\n") ; assert_eq ! (try_unescape ("hello\\u") , None) ; assert_eq ! (try_unescape ("hello\\u{") , None) ; assert_eq ! (try_unescape ("hello\\u{}") , None) ; assert_eq ! (try_unescape ("hello\\u{0}") . unwrap () , "hello\0") ; assert_eq ! (try_unescape ("hello\\u{000000}") . unwrap () , "hello\0") ; assert_eq ! (try_unescape ("hello\\u{0000000}") , None) ; } }
};
}
