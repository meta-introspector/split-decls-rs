// Generated macro for impl_160 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_160 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_160"}
// Dependencies: {}
impl < V : PartialOrd > LinspaceRoundingMethod < V > for Ceil < V > { fn search (values : & [V] , target : & V) -> Option < usize > { let ascending = if values . len () < 2 { true } else { values [0] . partial_cmp (& values [1]) == Some (Ordering :: Less) } ; match values . binary_search_by (| probe | { if ascending { probe . partial_cmp (target) . unwrap () } else { target . partial_cmp (probe) . unwrap () } }) { Ok (idx) => Some (idx) , Err (idx) => { let offset = if ascending { 0 } else { 1 } ; if idx < offset || idx >= values . len () + offset { return None ; } Some (idx - offset) } } } }
};
}
