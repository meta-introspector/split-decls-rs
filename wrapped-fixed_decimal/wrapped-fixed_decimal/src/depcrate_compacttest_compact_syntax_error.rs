// Generated macro for test_compact_syntax_error (function)
macro_rules! Depcrate_compacttest_compact_syntax_error {
() => {
// Module: crate::compact
// Provides: {"test_compact_syntax_error"}
// Dependencies: {}
# [test] fn test_compact_syntax_error () { # [derive (Debug)] struct TestCase { pub input_str : & 'static str , pub expected_err : Option < ParseError > , } let cases = [TestCase { input_str : "-123e4" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-123c" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1c10" , expected_err : None , } , TestCase { input_str : "1E1c1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1e1c1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1c1e1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "1c1E1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-1c01" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-1c-1" , expected_err : Some (ParseError :: Syntax) , } , TestCase { input_str : "-1c1" , expected_err : None , } ,] ; for cas in & cases { match CompactDecimal :: from_str (cas . input_str) { Ok (dec) => { assert_eq ! (cas . expected_err , None , "{cas:?}") ; assert_eq ! (cas . input_str , dec . to_string () , "{cas:?}") ; } Err (err) => { assert_eq ! (cas . expected_err , Some (err) , "{cas:?}") ; } } } }
};
}
