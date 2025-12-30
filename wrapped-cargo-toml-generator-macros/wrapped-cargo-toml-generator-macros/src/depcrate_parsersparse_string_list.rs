// Generated macro for parse_string_list (function)
macro_rules! Depcrate_parsersparse_string_list {
() => {
// Module: crate::parsers
// Provides: {"parse_string_list"}
// Dependencies: {}
pub fn parse_string_list (input : proc_macro2 :: TokenStream) -> Result < Vec < String > > { let string_list : BracketedStringList = syn :: parse2 (input) ? ; Ok (string_list . list . into_iter () . map (| s | s . value ()) . collect ()) }
};
}
