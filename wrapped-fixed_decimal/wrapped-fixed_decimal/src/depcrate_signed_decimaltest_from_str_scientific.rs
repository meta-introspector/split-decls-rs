// Generated macro for test_from_str_scientific (function)
macro_rules! Depcrate_signed_decimaltest_from_str_scientific {
() => {
// Module: crate::signed_decimal
// Provides: {"test_from_str_scientific"}
// Dependencies: {}
# [test] fn test_from_str_scientific () { # [derive (Debug)] struct TestCase { pub input_str : & 'static str , pub output : & 'static str , } let cases = [TestCase { input_str : "-5.4e10" , output : "-54000000000" , } , TestCase { input_str : "5.4e-2" , output : "0.054" , } , TestCase { input_str : "54.1e-2" , output : "0.541" , } , TestCase { input_str : "-541e-2" , output : "-5.41" , } , TestCase { input_str : "0.009E10" , output : "90000000" , } , TestCase { input_str : "-9000E-10" , output : "-0.0000009" , } ,] ; for cas in & cases { let input_str_roundtrip = Decimal :: from_str (cas . input_str) . unwrap () . to_string () ; assert_eq ! (cas . output , input_str_roundtrip) ; } }
};
}
