// Generated macro for impl_164 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_164 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_164"}
// Dependencies: {}
impl < V , S > LinspaceRoundingMethod < V > for Round < V , S > where V : Add < S , Output = V > + PartialOrd + Sub < V , Output = S > + Clone , S : PartialOrd + Clone , { fn search (values : & [V] , target : & V) -> Option < usize > { let ascending = if values . len () < 2 { true } else { values [0] . partial_cmp (& values [1]) == Some (Ordering :: Less) } ; match values . binary_search_by (| probe | { if ascending { probe . partial_cmp (target) . unwrap () } else { target . partial_cmp (probe) . unwrap () } }) { Ok (idx) => Some (idx) , Err (idx) => { if idx == 0 { return Some (0) ; } if idx == values . len () { return Some (idx - 1) ; } let left_delta = if ascending { target . clone () - values [idx - 1] . clone () } else { values [idx - 1] . clone () - target . clone () } ; let right_delta = if ascending { values [idx] . clone () - target . clone () } else { target . clone () - values [idx] . clone () } ; if left_delta . partial_cmp (& right_delta) == Some (Ordering :: Less) { Some (idx - 1) } else { Some (idx) } } } } }
};
}
