// Generated macro for parse_arguments (function)
macro_rules! Depcrate_parser_documentparse_arguments {
() => {
// Module: crate::parser::document
// Provides: {"parse_arguments"}
// Dependencies: {}
fn parse_arguments < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > , arguments : Option < & [Argument < S >] > ,) -> OptionParseResult < Arguments < 'a , S > > where S : ScalarValue , { if parser . peek () . item != Token :: ParenOpen { Ok (None) } else { Ok (Some (parser . delimited_nonempty_list (& Token :: ParenOpen , | p | parse_argument (p , schema , arguments) , & Token :: ParenClose ,) ? . map (| args | Arguments { items : args . into_iter () . map (| s | s . item) . collect () , }) ,)) } }
};
}
