// Generated macro for impl_30 (impl)
macro_rules! Depcrate_common_descriptionimpl_30 {
() => {
// Module: crate::common::description
// Provides: {"impl_30"}
// Dependencies: {}
impl Parse for Description { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { input . parse :: < syn :: LitStr > () . map (Self) } }
};
}
