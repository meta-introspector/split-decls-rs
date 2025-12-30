// Generated macro for boolean_struct_name (function)
macro_rules! Depcrate_de_testsboolean_struct_name {
() => {
// Module: crate::de::tests
// Provides: {"boolean_struct_name"}
// Dependencies: {}
# [test] fn boolean_struct_name () { check_from_str_bytes_reader :: < bool > ("true_" , Err (SpannedError { code : Error :: ExpectedBoolean , span : Span { start : Position { line : 1 , col : 1 } , end : Position { line : 1 , col : 1 } , } , }) ,) ; check_from_str_bytes_reader :: < bool > ("false_" , Err (SpannedError { code : Error :: ExpectedBoolean , span : Span { start : Position { line : 1 , col : 1 } , end : Position { line : 1 , col : 1 } , } , }) ,) ; }
};
}
