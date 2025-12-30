// Generated macro for impl_32 (impl)
macro_rules! Depcrate_parsersimpl_32 {
() => {
// Module: crate::parsers
// Provides: {"impl_32"}
// Dependencies: {}
impl Parse for RootInput { fn parse (input : ParseStream) -> Result < Self > { Ok (RootInput { items : Punctuated :: parse_terminated (input) ? , }) } }
};
}
