// Generated macro for find_streak (function)
macro_rules! Depcrate_slice_sortfind_streak {
() => {
// Module: crate::slice::sort
// Provides: {"find_streak"}
// Dependencies: {}
# [doc = " Finds a streak of presorted elements starting at the beginning of the slice. Returns the first"] # [doc = " value that is not part of said streak, and a bool denoting whether the streak was reversed."] # [doc = " Streaks can be increasing or decreasing."] fn find_streak < T , F > (v : & [T] , is_less : & F) -> (usize , bool) where F : Fn (& T , & T) -> bool , { let len = v . len () ; if len < 2 { return (len , false) ; } let mut end = 2 ; unsafe { let assume_reverse = is_less (v . get_unchecked (1) , v . get_unchecked (0)) ; if assume_reverse { while end < len && is_less (v . get_unchecked (end) , v . get_unchecked (end - 1)) { end += 1 ; } (end , true) } else { while end < len && ! is_less (v . get_unchecked (end) , v . get_unchecked (end - 1)) { end += 1 ; } (end , false) } } }
};
}
