// Generated macro for test_basic (function)
macro_rules! Depcrate_decimaltest_basic {
() => {
// Module: crate::decimal
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { # [derive (Debug)] struct TestCase { pub input : usize , pub delta : i16 , pub expected : & 'static str , } let cases = [TestCase { input : 51423 , delta : 0 , expected : "51423" , } , TestCase { input : 51423 , delta : - 2 , expected : "514.23" , } , TestCase { input : 51423 , delta : - 5 , expected : "0.51423" , } , TestCase { input : 51423 , delta : - 8 , expected : "0.00051423" , } , TestCase { input : 51423 , delta : 3 , expected : "51423000" , } , TestCase { input : 0 , delta : 0 , expected : "0" , } , TestCase { input : 0 , delta : - 2 , expected : "0.00" , } , TestCase { input : 0 , delta : 3 , expected : "0000" , } , TestCase { input : 500 , delta : 0 , expected : "500" , } , TestCase { input : 500 , delta : - 1 , expected : "50.0" , } , TestCase { input : 500 , delta : - 2 , expected : "5.00" , } , TestCase { input : 500 , delta : - 3 , expected : "0.500" , } , TestCase { input : 500 , delta : - 4 , expected : "0.0500" , } , TestCase { input : 500 , delta : 3 , expected : "500000" , } ,] ; for cas in & cases { let mut dec : UnsignedDecimal = cas . input . into () ; dec . multiply_pow10 (cas . delta) ; writeable :: assert_writeable_eq ! (dec , cas . expected , "{:?}" , cas) ; } }
};
}
