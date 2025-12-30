// Generated macro for quicksort (function)
macro_rules! Depcrate_sortquicksort {
() => {
// Module: crate::sort
// Provides: {"quicksort"}
// Dependencies: {}
pub (crate) fn quicksort < A , F > (vector : FocusMut < '_ , A > , cmp : & F) where A : Clone , F : Fn (& A , & A) -> Ordering , { let mut rng = rand_xoshiro :: Xoshiro256Plus :: seed_from_u64 (0) ; do_quicksort (vector , cmp , & mut rng) ; }
};
}
