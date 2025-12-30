// Generated macro for parse_variable_literal (function)
macro_rules! Depcrate_parser_valueparse_variable_literal {
() => {
// Module: crate::parser::value
// Provides: {"parse_variable_literal"}
// Dependencies: {}
fn parse_variable_literal < S > (parser : & mut Parser < '_ >) -> ParseResult < InputValue < S > > where S : ScalarValue , { let start_pos = & parser . expect (& Token :: Dollar) ? . span . start ; let Spanning { item : name , span : end_span , .. } = parser . expect_name () ? ; Ok (Spanning :: start_end (start_pos , & end_span . end , InputValue :: variable (name) ,)) }
};
}
