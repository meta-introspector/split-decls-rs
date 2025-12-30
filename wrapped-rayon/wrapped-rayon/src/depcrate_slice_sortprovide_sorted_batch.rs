// Generated macro for provide_sorted_batch (function)
macro_rules! Depcrate_slice_sortprovide_sorted_batch {
() => {
// Module: crate::slice::sort
// Provides: {"provide_sorted_batch"}
// Dependencies: {}
# [doc = " Takes a range as denoted by start and end, that is already sorted and extends it to the right if"] # [doc = " necessary with sorts optimized for smaller ranges such as insertion sort."] fn provide_sorted_batch < T , F > (v : & mut [T] , start : usize , mut end : usize , is_less : & F) -> usize where F : Fn (& T , & T) -> bool , { let len = v . len () ; assert ! (end >= start && end <= len) ; const MIN_INSERTION_RUN : usize = 10 ; let start_end_diff = end - start ; if start_end_diff < MIN_INSERTION_RUN && end < len { end = cmp :: min (start + MIN_INSERTION_RUN , len) ; let presorted_start = cmp :: max (start_end_diff , 1) ; insertion_sort_shift_left (& mut v [start .. end] , presorted_start , is_less) ; } end }
};
}
