// Generated macro for impl_158 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_158 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_158"}
// Dependencies: {}
impl < V : PartialOrd > LinspaceRoundingMethod < V > for Exact < V > { fn search (values : & [V] , target : & V) -> Option < usize > { values . iter () . position (| x | target == x) } }
};
}
