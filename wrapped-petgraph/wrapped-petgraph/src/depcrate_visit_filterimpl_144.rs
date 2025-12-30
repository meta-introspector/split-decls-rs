// Generated macro for impl_144 (impl)
macro_rules! Depcrate_visit_filterimpl_144 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_144"}
// Dependencies: {}
impl < F , G > NodeFiltered < G , F > where G : GraphBase , F : Fn (G :: NodeId) -> bool , { # [doc = " Create an `NodeFiltered` adaptor from the closure `filter`."] pub fn from_fn (graph : G , filter : F) -> Self { NodeFiltered (graph , filter) } }
};
}
