// Generated macro for test_basic (function)
macro_rules! Depcrate_signed_decimaltest_basic {
() => {
// Module: crate::signed_decimal
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { # [derive (Debug)] struct TestCase { pub input : isize , pub delta : i16 , pub expected : & 'static str , } let cases = [TestCase { input : 51423 , delta : 0 , expected : "51423" , } , TestCase { input : 51423 , delta : - 2 , expected : "514.23" , } , TestCase { input : 51423 , delta : - 5 , expected : "0.51423" , } , TestCase { input : 51423 , delta : - 8 , expected : "0.00051423" , } , TestCase { input : 51423 , delta : 3 , expected : "51423000" , } , TestCase { input : 0 , delta : 0 , expected : "0" , } , TestCase { input : 0 , delta : - 2 , expected : "0.00" , } , TestCase { input : 0 , delta : 3 , expected : "0000" , } , TestCase { input : 500 , delta : 0 , expected : "500" , } , TestCase { input : 500 , delta : - 1 , expected : "50.0" , } , TestCase { input : 500 , delta : - 2 , expected : "5.00" , } , TestCase { input : 500 , delta : - 3 , expected : "0.500" , } , TestCase { input : 500 , delta : - 4 , expected : "0.0500" , } , TestCase { input : 500 , delta : 3 , expected : "500000" , } , TestCase { input : - 123 , delta : 0 , expected : "-123" , } , TestCase { input : - 123 , delta : - 2 , expected : "-1.23" , } , TestCase { input : - 123 , delta : - 5 , expected : "-0.00123" , } , TestCase { input : - 123 , delta : 3 , expected : "-123000" , } ,] ; for cas in & cases { let mut dec : Decimal = cas . input . into () ; dec . absolute . multiply_pow10 (cas . delta) ; writeable :: assert_writeable_eq ! (dec , cas . expected , "{:?}" , cas) ; } }
};
}
