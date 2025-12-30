// Generated macro for impl_171 (impl)
macro_rules! Depcrate_visit_filterimpl_171 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_171"}
// Dependencies: {}
impl < F , G > EdgeFiltered < G , F > where G : IntoEdgeReferences , F : Fn (G :: EdgeRef) -> bool , { # [doc = " Create an `EdgeFiltered` adaptor from the closure `filter`."] pub fn from_fn (graph : G , filter : F) -> Self { EdgeFiltered (graph , filter) } }
};
}
