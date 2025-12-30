// Generated macro for test_repr (function)
macro_rules! Depcratetest_repr {
() => {
// Module: crate
// Provides: {"test_repr"}
// Dependencies: {}
# [test] fn test_repr () { unsafe { assert_eq ! (raw_byte_repr (& 17u8) , & [17]) ; assert_eq ! (raw_byte_repr ("abc") , "abc" . as_bytes ()) ; } }
};
}
