// Generated macro for parse_document_source (function)
macro_rules! Depcrate_parser_documentparse_document_source {
() => {
// Module: crate::parser::document
// Provides: {"parse_document_source"}
// Dependencies: {}
# [doc (hidden)] pub fn parse_document_source < 'a , S > (s : & 'a str , schema : & SchemaType < S > ,) -> UnlocatedParseResult < OwnedDocument < 'a , S > > where S : ScalarValue , { let mut lexer = Lexer :: new (s) ; let mut parser = Parser :: new (& mut lexer) . map_err (| s | s . map (Into :: into)) ? ; parse_document (& mut parser , schema) }
};
}
