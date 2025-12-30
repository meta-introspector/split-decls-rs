// Generated macro for impl_20 (impl)
macro_rules! Depcrate_common_deprecationimpl_20 {
() => {
// Module: crate::common::deprecation
// Provides: {"impl_20"}
// Dependencies: {}
impl Parse for Directive { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { Ok (Self { reason : input . try_parse :: < token :: Eq > () ? . map (| _ | input . parse :: < syn :: LitStr > ()) . transpose () ? , }) } }
};
}
