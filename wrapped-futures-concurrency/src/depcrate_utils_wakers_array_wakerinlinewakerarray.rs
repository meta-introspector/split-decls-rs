// Generated macro for InlineWakerArray (struct)
macro_rules! Depcrate_utils_wakers_array_wakerInlineWakerArray {
() => {
// Module: crate::utils::wakers::array::waker
// Provides: {"InlineWakerArray"}
// Dependencies: {}
# [doc = " An efficient waker which delegates wake events."] # [derive (Debug , Clone)] pub (crate) struct InlineWakerArray < const N : usize > { pub (crate) id : usize , pub (crate) readiness : Arc < Mutex < ReadinessArray < N > > > , }
};
}
