// Generated macro for parse_variables_definition (function)
macro_rules! Depcrate_parser_documentparse_variables_definition {
() => {
// Module: crate::parser::document
// Provides: {"parse_variables_definition"}
// Dependencies: {}
fn parse_variables_definition < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> OptionParseResult < VariablesDefinition < 'a , S > > where S : ScalarValue , { if parser . peek () . item != Token :: ParenOpen { Ok (None) } else { Ok (Some (parser . delimited_nonempty_list (& Token :: ParenOpen , | p | parse_variable_definition (p , schema) , & Token :: ParenClose ,) ? . map (| defs | VariablesDefinition { items : defs . into_iter () . map (| s | s . item) . collect () , }) ,)) } }
};
}
