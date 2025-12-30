// Generated macro for impl_162 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_162 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_162"}
// Dependencies: {}
impl < V : PartialOrd > LinspaceRoundingMethod < V > for Floor < V > { fn search (values : & [V] , target : & V) -> Option < usize > { let ascending = if values . len () < 2 { true } else { values [0] . partial_cmp (& values [1]) == Some (Ordering :: Less) } ; match values . binary_search_by (| probe | { if ascending { probe . partial_cmp (target) . unwrap () } else { target . partial_cmp (probe) . unwrap () } }) { Ok (idx) => Some (idx) , Err (idx) => { let offset = if ascending { 1 } else { 0 } ; if idx < offset || idx >= values . len () + offset { return None ; } Some (idx - offset) } } } }
};
}
