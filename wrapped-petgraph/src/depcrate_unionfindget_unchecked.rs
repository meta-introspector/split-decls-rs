// Generated macro for get_unchecked (function)
macro_rules! Depcrate_unionfindget_unchecked {
() => {
// Module: crate::unionfind
// Provides: {"get_unchecked"}
// Dependencies: {}
# [inline] unsafe fn get_unchecked < K > (xs : & [K] , index : usize) -> & K { unsafe { debug_assert ! (index < xs . len ()) ; xs . get_unchecked (index) } }
};
}
