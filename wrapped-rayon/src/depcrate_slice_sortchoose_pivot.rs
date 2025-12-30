// Generated macro for choose_pivot (function)
macro_rules! Depcrate_slice_sortchoose_pivot {
() => {
// Module: crate::slice::sort
// Provides: {"choose_pivot"}
// Dependencies: {}
# [doc = " Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted."] # [doc = ""] # [doc = " Elements in `v` might be reordered in the process."] fn choose_pivot < T , F > (v : & mut [T] , is_less : & F) -> (usize , bool) where F : Fn (& T , & T) -> bool , { const SHORTEST_MEDIAN_OF_MEDIANS : usize = 50 ; const MAX_SWAPS : usize = 4 * 3 ; let len = v . len () ; # [allow (clippy :: identity_op)] let mut a = len / 4 * 1 ; let mut b = len / 4 * 2 ; let mut c = len / 4 * 3 ; let mut swaps = 0 ; if len >= 8 { let mut sort2 = | a : & mut usize , b : & mut usize | unsafe { if is_less (v . get_unchecked (* b) , v . get_unchecked (* a)) { ptr :: swap (a , b) ; swaps += 1 ; } } ; let mut sort3 = | a : & mut usize , b : & mut usize , c : & mut usize | { sort2 (a , b) ; sort2 (b , c) ; sort2 (a , b) ; } ; if len >= SHORTEST_MEDIAN_OF_MEDIANS { let mut sort_adjacent = | a : & mut usize | { let tmp = * a ; sort3 (& mut (tmp - 1) , a , & mut (tmp + 1)) ; } ; sort_adjacent (& mut a) ; sort_adjacent (& mut b) ; sort_adjacent (& mut c) ; } sort3 (& mut a , & mut b , & mut c) ; } if swaps < MAX_SWAPS { (b , swaps == 0) } else { v . reverse () ; (len - 1 - b , true) } }
};
}
