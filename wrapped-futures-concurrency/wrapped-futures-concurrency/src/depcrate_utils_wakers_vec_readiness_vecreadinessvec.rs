// Generated macro for ReadinessVec (struct)
macro_rules! Depcrate_utils_wakers_vec_readiness_vecReadinessVec {
() => {
// Module: crate::utils::wakers::vec::readiness_vec
// Provides: {"ReadinessVec"}
// Dependencies: {}
# [doc = " Tracks which wakers are \"ready\" and should be polled."] # [derive (Debug)] pub (crate) struct ReadinessVec { ready_count : usize , max_count : usize , readiness_list : FixedBitSet , parent_waker : Option < Waker > , }
};
}
