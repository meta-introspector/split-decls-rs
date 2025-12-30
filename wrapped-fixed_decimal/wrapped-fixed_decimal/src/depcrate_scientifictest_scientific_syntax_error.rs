// Generated macro for test_scientific_syntax_error (function)
macro_rules! Depcrate_scientifictest_scientific_syntax_error {
() => {
// Module: crate::scientific
// Provides: {"test_scientific_syntax_error"}
// Dependencies: {}
# [test] fn test_scientific_syntax_error () { # [derive (Debug)] struct TestCase { pub input_str : & 'static str , pub expected_err : Option < ParseError > , } let cases = [TestCase { input_str : "5" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-123c4" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-123e" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1e10" , expected_err : None , } , TestCase { input_str : "1e1e1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1e1E1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1E1e1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-1e+01" , expected_err : None , } , TestCase { input_str : "-1e+1.0" , expected_err : Some (ParseError :: Limit) , } , TestCase { input_str : "-1e+-1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "123E4" , expected_err : Some (ParseError :: Syntax) , } ,] ; for cas in & cases { match ScientificDecimal :: from_str (cas . input_str) { Ok (dec) => { assert_eq ! (cas . expected_err , None , "{cas:?}") ; assert_eq ! (cas . input_str , dec . to_string () , "{cas:?}") ; } Err (err) => { assert_eq ! (cas . expected_err , Some (err) , "{cas:?}") ; } } } }
};
}
