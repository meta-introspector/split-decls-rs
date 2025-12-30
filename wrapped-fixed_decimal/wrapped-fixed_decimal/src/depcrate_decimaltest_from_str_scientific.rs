// Generated macro for test_from_str_scientific (function)
macro_rules! Depcrate_decimaltest_from_str_scientific {
() => {
// Module: crate::decimal
// Provides: {"test_from_str_scientific"}
// Dependencies: {}
# [test] fn test_from_str_scientific () { # [derive (Debug)] struct TestCase { pub input_str : & 'static str , pub output : & 'static str , } let cases = [TestCase { input_str : "5.4e-2" , output : "0.054" , } , TestCase { input_str : "54.1e-2" , output : "0.541" , } , TestCase { input_str : "0.009E10" , output : "90000000" , } ,] ; for cas in & cases { let input_str_roundtrip = UnsignedDecimal :: from_str (cas . input_str) . unwrap () . to_string () ; assert_eq ! (cas . output , input_str_roundtrip) ; } }
};
}
