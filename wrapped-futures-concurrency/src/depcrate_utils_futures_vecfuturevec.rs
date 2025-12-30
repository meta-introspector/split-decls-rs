// Generated macro for FutureVec (struct)
macro_rules! Depcrate_utils_futures_vecFutureVec {
() => {
// Module: crate::utils::futures::vec
// Provides: {"FutureVec"}
// Dependencies: {}
# [doc = " An array of futures which can be dropped in-place, intended to be"] # [doc = " constructed once and then accessed through pin projections."] pub (crate) struct FutureVec < T > { futures : Vec < ManuallyDrop < T > > , }
};
}
