// Generated macro for new_cap (function)
macro_rules! Depcrate_raw_vecnew_cap {
() => {
// Module: crate::raw_vec
// Provides: {"new_cap"}
// Dependencies: {}
# [doc = " `Cap(cap)`, except if `T` is a ZST then `Cap::ZERO`."] # [doc = ""] # [doc = " # Safety: cap must be <= `isize::MAX`."] unsafe fn new_cap < T > (cap : usize) -> Cap { if T :: IS_ZST { ZERO_CAP } else { unsafe { Cap :: new_unchecked (cap) } } }
};
}
