// Generated macro for binary_search (function)
macro_rules! Depcrate_treefrogbinary_search {
() => {
// Module: crate::treefrog
// Provides: {"binary_search"}
// Dependencies: {}
fn binary_search < T > (slice : & [T] , mut cmp : impl FnMut (& T) -> bool) -> usize { let mut hi = slice . len () ; let mut lo = 0 ; while lo < hi { let mid = lo + (hi - lo) / 2 ; if cmp (& slice [mid]) { lo = mid + 1 ; } else { hi = mid ; } } lo }
};
}
