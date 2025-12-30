// Generated macro for parse_list_literal (function)
macro_rules! Depcrate_parser_valueparse_list_literal {
() => {
// Module: crate::parser::value
// Provides: {"parse_list_literal"}
// Dependencies: {}
fn parse_list_literal < S > (parser : & mut Parser < '_ > , is_const : bool , schema : & SchemaType < S > , tpe : Option < & MetaType < S > > ,) -> ParseResult < InputValue < S > > where S : ScalarValue , { Ok (parser . delimited_list (& Token :: BracketOpen , | p | parse_value_literal (p , is_const , schema , tpe) , & Token :: BracketClose ,) ? . map (InputValue :: parsed_list)) }
};
}
