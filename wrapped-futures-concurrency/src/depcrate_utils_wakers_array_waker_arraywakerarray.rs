// Generated macro for WakerArray (struct)
macro_rules! Depcrate_utils_wakers_array_waker_arrayWakerArray {
() => {
// Module: crate::utils::wakers::array::waker_array
// Provides: {"WakerArray"}
// Dependencies: {}
# [doc = " A collection of wakers which delegate to an in-line waker."] pub (crate) struct WakerArray < const N : usize > { wakers : [Waker ; N] , readiness : Arc < Mutex < ReadinessArray < N > > > , }
};
}
