// Generated macro for maximal_suffix (function)
macro_rules! Depcrate_findmaximal_suffix {
() => {
// Module: crate::find
// Provides: {"maximal_suffix"}
// Dependencies: {}
fn maximal_suffix (arr : & [char] , order_greater : bool) -> (usize , usize) { let mut left = 0 ; let mut right = 1 ; let mut offset = 0 ; let mut period = 1 ; while let Some (& a) = arr . get (right + offset) { let b = arr [left + offset] ; if (a < b && ! order_greater) || (a > b && order_greater) { right += offset + 1 ; offset = 0 ; period = right - left ; } else if a == b { if offset + 1 == period { right += offset + 1 ; offset = 0 ; } else { offset += 1 ; } } else { left = right ; right += 1 ; offset = 0 ; period = 1 ; } } (left , period) }
};
}
