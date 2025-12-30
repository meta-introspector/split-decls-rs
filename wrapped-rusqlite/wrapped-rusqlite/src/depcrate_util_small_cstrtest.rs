// Generated macro for test (module)
macro_rules! Depcrate_util_small_cstrtest {
() => {
// Module: crate::util::small_cstr
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_small_cstring () { assert_eq ! (SmallCString :: default () . 0 , SmallCString :: new ("") . unwrap () . 0) ; assert_eq ! (SmallCString :: new ("foo") . unwrap () . len () , 3) ; assert_eq ! (SmallCString :: new ("foo") . unwrap () . as_bytes_with_nul () , b"foo\0") ; assert_eq ! (SmallCString :: new ("foo") . unwrap () . as_bytes_without_nul () , b"foo" ,) ; assert_eq ! (SmallCString :: new ("😀") . unwrap () . len () , 4) ; assert_eq ! (SmallCString :: new ("😀") . unwrap () . 0 . as_slice () , b"\xf0\x9f\x98\x80\0" ,) ; assert_eq ! (SmallCString :: new ("😀") . unwrap () . as_bytes_without_nul () , b"\xf0\x9f\x98\x80" ,) ; assert_eq ! (SmallCString :: new ("") . unwrap () . len () , 0) ; assert ! (SmallCString :: new ("") . unwrap () . is_empty ()) ; assert_eq ! (SmallCString :: new ("") . unwrap () . 0 . as_slice () , b"\0") ; assert_eq ! (SmallCString :: new ("") . unwrap () . as_bytes_without_nul () , b"") ; SmallCString :: new ("\0") . unwrap_err () ; SmallCString :: new ("\0abc") . unwrap_err () ; SmallCString :: new ("abc\0") . unwrap_err () ; } }
};
}
