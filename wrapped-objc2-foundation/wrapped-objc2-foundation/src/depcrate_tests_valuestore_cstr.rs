// Generated macro for store_cstr (function)
macro_rules! Depcrate_tests_valuestore_cstr {
() => {
// Module: crate::tests::value
// Provides: {"store_cstr"}
// Dependencies: {}
# [test] fn store_cstr () { let s = CStr :: from_bytes_with_nul (b"test123\0") . unwrap () ; let val = NSValue :: new (s . as_ptr ()) ; assert ! (val . contains_encoding ::<* const c_char > ()) ; let s2 = unsafe { CStr :: from_ptr (val . get ()) } ; assert_eq ! (s2 , s) ; }
};
}
