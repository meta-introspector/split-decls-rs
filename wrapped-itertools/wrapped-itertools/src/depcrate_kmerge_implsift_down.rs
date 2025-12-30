// Generated macro for sift_down (function)
macro_rules! Depcrate_kmerge_implsift_down {
() => {
// Module: crate::kmerge_impl
// Provides: {"sift_down"}
// Dependencies: {}
# [doc = " Sift down element at `index` (`heap` is a min-heap wrt the ordering)"] fn sift_down < T , S > (heap : & mut [T] , index : usize , mut less_than : S) where S : FnMut (& T , & T) -> bool , { debug_assert ! (index <= heap . len ()) ; let mut pos = index ; let mut child = 2 * pos + 1 ; while child + 1 < heap . len () { child += less_than (& heap [child + 1] , & heap [child]) as usize ; if ! less_than (& heap [child] , & heap [pos]) { return ; } heap . swap (pos , child) ; pos = child ; child = 2 * pos + 1 ; } if child + 1 == heap . len () && less_than (& heap [child] , & heap [pos]) { heap . swap (pos , child) ; } }
};
}
