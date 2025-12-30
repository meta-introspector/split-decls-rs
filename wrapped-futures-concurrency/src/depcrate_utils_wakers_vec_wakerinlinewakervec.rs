// Generated macro for InlineWakerVec (struct)
macro_rules! Depcrate_utils_wakers_vec_wakerInlineWakerVec {
() => {
// Module: crate::utils::wakers::vec::waker
// Provides: {"InlineWakerVec"}
// Dependencies: {}
# [doc = " An efficient waker which delegates wake events."] # [derive (Debug , Clone)] pub (crate) struct InlineWakerVec { pub (crate) id : usize , pub (crate) readiness : Arc < Mutex < ReadinessVec > > , }
};
}
