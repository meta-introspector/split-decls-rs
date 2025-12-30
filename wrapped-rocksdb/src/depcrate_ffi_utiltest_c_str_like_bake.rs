// Generated macro for test_c_str_like_bake (function)
macro_rules! Depcrate_ffi_utiltest_c_str_like_bake {
() => {
// Module: crate::ffi_util
// Provides: {"test_c_str_like_bake"}
// Dependencies: {}
# [test] fn test_c_str_like_bake () { fn test < S : CStrLike > (value : S) -> Result < usize , S :: Error > { value . bake () . map (| value | unsafe { libc :: strlen (value . as_ptr ()) }) } assert_eq ! (Ok (3) , test ("foo")) ; assert_eq ! (Ok (3) , test (& String :: from ("foo"))) ; assert_eq ! (Ok (3) , test (CString :: new ("foo") . unwrap () . as_ref ())) ; assert_eq ! (Ok (3) , test (& CString :: new ("foo") . unwrap ())) ; assert_eq ! (Ok (3) , test (CString :: new ("foo") . unwrap ())) ; assert_eq ! (3 , test ("foo\0bar") . err () . unwrap () . nul_position ()) ; }
};
}
