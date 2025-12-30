// Generated macro for test_hash_conversions (function)
macro_rules! Depcrate_testtest_hash_conversions {
() => {
// Module: crate::test
// Provides: {"test_hash_conversions"}
// Dependencies: {}
# [test] fn test_hash_conversions () { let bytes1 = [42 ; 32] ; let hash1 : crate :: Hash = bytes1 . into () ; let bytes2 : [u8 ; 32] = hash1 . into () ; assert_eq ! (bytes1 , bytes2) ; let bytes3 = * hash1 . as_bytes () ; assert_eq ! (bytes1 , bytes3) ; let hash2 = crate :: Hash :: from_bytes (bytes1) ; assert_eq ! (hash1 , hash2) ; let hex = hash1 . to_hex () ; let hash3 = crate :: Hash :: from_hex (hex . as_bytes ()) . unwrap () ; assert_eq ! (hash1 , hash3) ; let slice1 : & [u8] = bytes1 . as_slice () ; let hash4 = crate :: Hash :: from_slice (slice1) . expect ("correct length") ; assert_eq ! (hash1 , hash4) ; assert ! (crate :: Hash :: from_slice (& []) . is_err ()) ; assert ! (crate :: Hash :: from_slice (& [42]) . is_err ()) ; assert ! (crate :: Hash :: from_slice ([42 ; 31] . as_slice ()) . is_err ()) ; assert ! (crate :: Hash :: from_slice ([42 ; 33] . as_slice ()) . is_err ()) ; assert ! (crate :: Hash :: from_slice ([42 ; 100] . as_slice ()) . is_err ()) ; }
};
}
