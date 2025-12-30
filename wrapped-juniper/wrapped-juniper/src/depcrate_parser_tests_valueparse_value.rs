// Generated macro for parse_value (function)
macro_rules! Depcrate_parser_tests_valueparse_value {
() => {
// Module: crate::parser::tests::value
// Provides: {"parse_value"}
// Dependencies: {}
fn parse_value < S > (s : & str , meta : & MetaType < S >) -> Spanning < InputValue < S > > where S : ScalarValue , { let mut lexer = Lexer :: new (s) ; let mut parser = Parser :: new (& mut lexer) . unwrap_or_else (| _ | panic ! ("Lexer error on input {s:#?}")) ; let schema = SchemaType :: new :: < Query , EmptyMutation < () > , EmptySubscription < () > > (& () , & () , & ()) ; parse_value_literal (& mut parser , false , & schema , Some (meta)) . unwrap_or_else (| _ | panic ! ("Parse error on input {s:#?}")) }
};
}
