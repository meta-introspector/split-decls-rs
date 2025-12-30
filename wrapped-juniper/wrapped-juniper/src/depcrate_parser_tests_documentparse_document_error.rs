// Generated macro for parse_document_error (function)
macro_rules! Depcrate_parser_tests_documentparse_document_error {
() => {
// Module: crate::parser::tests::document
// Provides: {"parse_document_error"}
// Dependencies: {}
fn parse_document_error < S : ScalarValue > (s : & str) -> Spanning < ParseError > { match parse_document_source :: < S > (s , & SchemaType :: new :: < QueryRoot , MutationRoot , SubscriptionRoot > (& () , & () , & ()) ,) { Ok (doc) => panic ! ("*No* parse error on input {s:#?} =>\n{doc:#?}") , Err (err) => err , } }
};
}
