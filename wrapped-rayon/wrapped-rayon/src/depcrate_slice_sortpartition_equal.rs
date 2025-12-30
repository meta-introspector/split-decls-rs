// Generated macro for partition_equal (function)
macro_rules! Depcrate_slice_sortpartition_equal {
() => {
// Module: crate::slice::sort
// Provides: {"partition_equal"}
// Dependencies: {}
# [doc = " Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`."] # [doc = ""] # [doc = " Returns the number of elements equal to the pivot. It is assumed that `v` does not contain"] # [doc = " elements smaller than the pivot."] fn partition_equal < T , F > (v : & mut [T] , pivot : usize , is_less : & F) -> usize where F : Fn (& T , & T) -> bool , { v . swap (0 , pivot) ; let (pivot , v) = v . split_at_mut (1) ; let pivot = & mut pivot [0] ; let tmp = mem :: ManuallyDrop :: new (unsafe { ptr :: read (pivot) }) ; let _pivot_guard = InsertionHole { src : & * tmp , dest : pivot , } ; let pivot = & * tmp ; let len = v . len () ; if len == 0 { return 0 ; } let mut l = 0 ; let mut r = len ; loop { unsafe { while l < r && ! is_less (pivot , v . get_unchecked (l)) { l += 1 ; } loop { r -= 1 ; if l >= r || ! is_less (pivot , v . get_unchecked (r)) { break ; } } if l >= r { break ; } let ptr = v . as_mut_ptr () ; ptr :: swap (ptr . add (l) , ptr . add (r)) ; l += 1 ; } } l + 1 }
};
}
