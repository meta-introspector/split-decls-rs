// Generated macro for test_unescape_raw_str (function)
macro_rules! Depcrate_teststest_unescape_raw_str {
() => {
// Module: crate::tests
// Provides: {"test_unescape_raw_str"}
// Dependencies: {}
# [test] fn test_unescape_raw_str () { fn check (literal : & str , expected : & [(Range < usize > , Result < char , EscapeError >)]) { let mut unescaped = Vec :: with_capacity (literal . len ()) ; check_raw_str (literal , | range , res | unescaped . push ((range , res))) ; assert_eq ! (unescaped , expected) ; } check ("\r" , & [(0 .. 1 , Err (EscapeError :: BareCarriageReturnInRawString))] ,) ; check ("\rx" , & [(0 .. 1 , Err (EscapeError :: BareCarriageReturnInRawString)) , (1 .. 2 , Ok ('x')) ,] ,) ; }
};
}
