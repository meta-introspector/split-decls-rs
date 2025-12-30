// Generated macro for partition (function)
macro_rules! Depcrate_slice_sortpartition {
() => {
// Module: crate::slice::sort
// Provides: {"partition"}
// Dependencies: {}
# [doc = " Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or"] # [doc = " equal to `v[pivot]`."] # [doc = ""] # [doc = " Returns a tuple of:"] # [doc = ""] # [doc = " 1. Number of elements smaller than `v[pivot]`."] # [doc = " 2. True if `v` was already partitioned."] fn partition < T , F > (v : & mut [T] , pivot : usize , is_less : & F) -> (usize , bool) where F : Fn (& T , & T) -> bool , { let (mid , was_partitioned) = { v . swap (0 , pivot) ; let (pivot , v) = v . split_at_mut (1) ; let pivot = & mut pivot [0] ; let tmp = mem :: ManuallyDrop :: new (unsafe { ptr :: read (pivot) }) ; let _pivot_guard = InsertionHole { src : & * tmp , dest : pivot , } ; let pivot = & * tmp ; let mut l = 0 ; let mut r = v . len () ; unsafe { while l < r && is_less (v . get_unchecked (l) , pivot) { l += 1 ; } while l < r && ! is_less (v . get_unchecked (r - 1) , pivot) { r -= 1 ; } } (l + partition_in_blocks (& mut v [l .. r] , pivot , is_less) , l >= r ,) } ; v . swap (0 , mid) ; (mid , was_partitioned) }
};
}
