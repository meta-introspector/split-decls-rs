// Generated macro for test_decimal_encoding (function)
macro_rules! Depcrate_tests_decimal_numbertest_decimal_encoding {
() => {
// Module: crate::tests::decimal_number
// Provides: {"test_decimal_encoding"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "has different encoding, yet unsupported")] fn test_decimal_encoding () { let decimal = NSDecimal { _inner : 0 , _mantissa : [0 ; 8] , } ; let obj = NSDecimalNumber :: initWithDecimal (NSDecimalNumber :: alloc () , decimal) ; assert_eq ! (decimal , obj . decimalValue ()) ; }
};
}
