// Generated macro for impl_34 (impl)
macro_rules! Depcrate_parsersimpl_34 {
() => {
// Module: crate::parsers
// Provides: {"impl_34"}
// Dependencies: {}
impl Parse for TomlSection { fn parse (input : ParseStream) -> Result < Self > { Ok (TomlSection { items : Punctuated :: parse_terminated (input) ? , }) } }
};
}
