// Generated macro for ReadinessArray (struct)
macro_rules! Depcrate_utils_wakers_array_readiness_arrayReadinessArray {
() => {
// Module: crate::utils::wakers::array::readiness_array
// Provides: {"ReadinessArray"}
// Dependencies: {}
# [doc = " Tracks which wakers are \"ready\" and should be polled."] # [derive (Debug)] pub (crate) struct ReadinessArray < const N : usize > { count : usize , readiness_list : [bool ; N] , parent_waker : Option < Waker > , }
};
}
