// Generated macro for impl_36 (impl)
macro_rules! Depcrate_parsersimpl_36 {
() => {
// Module: crate::parsers
// Provides: {"impl_36"}
// Dependencies: {}
impl Parse for BracketedStringList { fn parse (input : ParseStream) -> Result < Self > { let content ; bracketed ! (content in input) ; Ok (BracketedStringList { list : Punctuated :: parse_terminated (& content) ? , }) } }
};
}
