// Generated macro for impl_set_inaccessible (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_inaccessible {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_inaccessible"}
// Dependencies: {}
macro_rules ! impl_set_inaccessible { () => { # [doc = " Indicate that an enum is not accessible from a supergraph when using"] # [doc = " Apollo Federation"] # [doc = ""] # [doc = " Reference: <https://www.apollographql.com/docs/federation/federated-types/federated-directives/#inaccessible>"] # [inline] pub fn inaccessible (self) -> Self { Self { inaccessible : true , .. self } } } ; }
};
}
