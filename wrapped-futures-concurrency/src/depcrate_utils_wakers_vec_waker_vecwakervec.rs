// Generated macro for WakerVec (struct)
macro_rules! Depcrate_utils_wakers_vec_waker_vecWakerVec {
() => {
// Module: crate::utils::wakers::vec::waker_vec
// Provides: {"WakerVec"}
// Dependencies: {}
# [doc = " A collection of wakers which delegate to an in-line waker."] pub (crate) struct WakerVec { wakers : Vec < Waker > , readiness : Arc < Mutex < ReadinessVec > > , }
};
}
