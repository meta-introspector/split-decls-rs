// Generated macro for k_smallest_relaxed_general (function)
macro_rules! Depcrate_k_smallestk_smallest_relaxed_general {
() => {
// Module: crate::k_smallest
// Provides: {"k_smallest_relaxed_general"}
// Dependencies: {}
pub (crate) fn k_smallest_relaxed_general < I , F > (iter : I , k : usize , mut comparator : F) -> Vec < I :: Item > where I : Iterator , F : FnMut (& I :: Item , & I :: Item) -> Ordering , { if k == 0 { iter . last () ; return Vec :: new () ; } let mut iter = iter . fuse () ; let mut buf = iter . by_ref () . take (2 * k) . collect :: < Vec < _ > > () ; if buf . len () < k { buf . sort_unstable_by (& mut comparator) ; return buf ; } buf . select_nth_unstable_by (k - 1 , & mut comparator) ; buf . truncate (k) ; iter . for_each (| val | { if comparator (& val , & buf [k - 1]) != Ordering :: Less { return ; } assert_ne ! (buf . len () , buf . capacity ()) ; buf . push (val) ; if buf . len () == 2 * k { buf . select_nth_unstable_by (k - 1 , & mut comparator) ; buf . truncate (k) ; } }) ; buf . sort_unstable_by (& mut comparator) ; buf . truncate (k) ; buf }
};
}
