// Generated macro for get_unchecked_mut (function)
macro_rules! Depcrate_unionfindget_unchecked_mut {
() => {
// Module: crate::unionfind
// Provides: {"get_unchecked_mut"}
// Dependencies: {}
# [inline] unsafe fn get_unchecked_mut < K > (xs : & mut [K] , index : usize) -> & mut K { unsafe { debug_assert ! (index < xs . len ()) ; xs . get_unchecked_mut (index) } }
};
}
