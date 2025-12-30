// Generated macro for impl_set_tags (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_tags {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_tags"}
// Dependencies: {}
macro_rules ! impl_set_tags { () => { # [doc = " Arbitrary string metadata that will be propagated to the supergraph"] # [doc = " when using Apollo Federation. This attribute is repeatable"] # [doc = ""] # [doc = " Reference: <https://www.apollographql.com/docs/federation/federated-types/federated-directives/#applying-metadata>"] # [inline] pub fn tags < I : IntoIterator < Item = T >, T : Into < String >> (self , tags : I) -> Self { Self { tags : tags . into_iter () . map (Into :: into) . collect () , .. self } } } ; }
};
}
