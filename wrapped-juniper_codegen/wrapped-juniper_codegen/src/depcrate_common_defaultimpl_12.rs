// Generated macro for impl_12 (impl)
macro_rules! Depcrate_common_defaultimpl_12 {
() => {
// Module: crate::common::default
// Provides: {"impl_12"}
// Dependencies: {}
impl Parse for Value { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { Ok (input . try_parse :: < token :: Eq > () ? . map (| _ | input . parse :: < syn :: Expr > ()) . transpose () ? . into ()) } }
};
}
