// Generated macro for impl_37 (impl)
macro_rules! Depcrate_parseimpl_37 {
() => {
// Module: crate::parse
// Provides: {"impl_37"}
// Dependencies: {}
impl Parse for AttributeMap { fn parse (input : ParseStream) -> syn :: Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; Ok (Self { meta : attrs . into_iter () . map (| a | a . meta) . collect () , names : parse_ident_array (input) ? , }) } }
};
}
