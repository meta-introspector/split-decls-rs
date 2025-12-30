// Generated macro for binary_search (function)
macro_rules! Depcrate_helpersbinary_search {
() => {
// Module: crate::helpers
// Provides: {"binary_search"}
// Dependencies: {}
pub (crate) fn binary_search (mut l : f64 , mut h : f64 , test : impl Fn (f64) -> bool , epsilon : f64 ,) -> f64 { debug_assert ! (l < h) ; loop { let mid = l + (h - l) / 2.0 ; (l , h) = if test (mid) { (l , mid) } else { (mid , h) } ; if (h - l) < epsilon { return mid ; } } }
};
}
