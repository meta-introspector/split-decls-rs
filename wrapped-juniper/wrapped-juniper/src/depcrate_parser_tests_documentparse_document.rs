// Generated macro for parse_document (function)
macro_rules! Depcrate_parser_tests_documentparse_document {
() => {
// Module: crate::parser::tests::document
// Provides: {"parse_document"}
// Dependencies: {}
fn parse_document < S > (s : & str) -> ast :: OwnedDocument < '_ , S > where S : ScalarValue , { parse_document_source (s , & SchemaType :: new :: < QueryRoot , MutationRoot , SubscriptionRoot > (& () , & () , & ()) ,) . unwrap_or_else (| e | panic ! ("parse error on input {s:#?}:\n{e}")) }
};
}
