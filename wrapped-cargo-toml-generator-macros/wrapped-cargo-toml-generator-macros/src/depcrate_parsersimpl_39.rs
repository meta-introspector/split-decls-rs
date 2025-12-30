// Generated macro for impl_39 (impl)
macro_rules! Depcrate_parsersimpl_39 {
() => {
// Module: crate::parsers
// Provides: {"impl_39"}
// Dependencies: {}
impl Parse for InlineTable { fn parse (input : ParseStream) -> Result < Self > { let content ; braced ! (content in input) ; Ok (InlineTable { items : Punctuated :: parse_terminated (& content) ? , }) } }
};
}
