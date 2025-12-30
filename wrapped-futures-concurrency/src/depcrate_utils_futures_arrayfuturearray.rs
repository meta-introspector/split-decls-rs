// Generated macro for FutureArray (struct)
macro_rules! Depcrate_utils_futures_arrayFutureArray {
() => {
// Module: crate::utils::futures::array
// Provides: {"FutureArray"}
// Dependencies: {}
# [doc = " An array of futures which can be dropped in-place, intended to be"] # [doc = " constructed once and then accessed through pin projections."] pub (crate) struct FutureArray < T , const N : usize > { futures : [ManuallyDrop < T > ; N] , }
};
}
