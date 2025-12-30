// Generated macro for errors (function)
macro_rules! Depcrate_parser_tests_documenterrors {
() => {
// Module: crate::parser::tests::document
// Provides: {"errors"}
// Dependencies: {}
# [test] fn errors () { assert_eq ! (parse_document_error ::< DefaultScalarValue > ("{") , Spanning :: zero_width (& SourcePosition :: new (1 , 0 , 1) , ParseError :: UnexpectedEndOfFile)) ; assert_eq ! (parse_document_error ::< DefaultScalarValue > ("{ ...MissingOn }\nfragment MissingOn Type") , Spanning :: start_end (& SourcePosition :: new (36 , 1 , 19) , & SourcePosition :: new (40 , 1 , 23) , ParseError :: UnexpectedToken ("Type" . into ()))) ; assert_eq ! (parse_document_error ::< DefaultScalarValue > ("{ ...on }") , Spanning :: start_end (& SourcePosition :: new (8 , 0 , 8) , & SourcePosition :: new (9 , 0 , 9) , ParseError :: unexpected_token (Token :: CurlyClose))) ; assert_eq ! (parse_document_error ::< DefaultScalarValue > (r#""description" { foo }"#) , Spanning :: start_end (& SourcePosition :: new (14 , 0 , 14) , & SourcePosition :: new (15 , 0 , 15) , ParseError :: unexpected_token (Token :: CurlyOpen)) ,) ; }
};
}
