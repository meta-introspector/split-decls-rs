// Generated macro for realpath_test (function)
macro_rules! Depcraterealpath_test {
() => {
// Module: crate
// Provides: {"realpath_test"}
// Dependencies: {}
# [test] # [cfg (windows)] fn realpath_test () { assert_eq ! (r"C:\WINDOWS" , canonicalize (r"C:\Windows") . unwrap () . to_str () . unwrap () . to_uppercase ()) ; assert_ne ! (r"." , canonicalize (r".") . unwrap () . to_str () . unwrap ()) ; }
};
}
