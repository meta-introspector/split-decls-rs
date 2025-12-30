// Generated macro for test_as_ref_reader (function)
macro_rules! Depcrate_decodetest_as_ref_reader {
() => {
// Module: crate::decode
// Provides: {"test_as_ref_reader"}
// Dependencies: {}
# [test] fn test_as_ref_reader () { let buf = [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10] ; let mut rd = ReadRefReader :: new (& buf) ; assert_eq ! (rd . read_slice (1) . unwrap () , Reference :: Borrowed (& [0] [..])) ; assert_eq ! (rd . read_slice (6) . unwrap () , Reference :: Borrowed (& [1 , 2 , 3 , 4 , 5 , 6] [..])) ; assert ! (rd . read_slice (5) . is_err ()) ; assert_eq ! (rd . read_slice (4) . unwrap () , Reference :: Borrowed (& [7 , 8 , 9 , 10] [..])) ; }
};
}
