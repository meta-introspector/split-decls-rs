// Generated macro for test_c_str_like_into (function)
macro_rules! Depcrate_ffi_utiltest_c_str_like_into {
() => {
// Module: crate::ffi_util
// Provides: {"test_c_str_like_into"}
// Dependencies: {}
# [test] fn test_c_str_like_into () { fn test < S : CStrLike > (value : S) -> Result < CString , S :: Error > { value . into_c_string () } let want = CString :: new ("foo") . unwrap () ; assert_eq ! (Ok (want . clone ()) , test ("foo")) ; assert_eq ! (Ok (want . clone ()) , test (& String :: from ("foo"))) ; assert_eq ! (Ok (want . clone ()) , test (CString :: new ("foo") . unwrap () . as_ref ())) ; assert_eq ! (Ok (want . clone ()) , test (& CString :: new ("foo") . unwrap ())) ; assert_eq ! (Ok (want) , test (CString :: new ("foo") . unwrap ())) ; assert_eq ! (3 , test ("foo\0bar") . err () . unwrap () . nul_position ()) ; }
};
}
