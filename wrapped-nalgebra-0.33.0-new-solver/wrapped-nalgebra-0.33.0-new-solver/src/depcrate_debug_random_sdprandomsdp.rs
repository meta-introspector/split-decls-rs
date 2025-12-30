// Generated macro for RandomSDP (struct)
macro_rules! Depcrate_debug_random_sdpRandomSDP {
() => {
// Module: crate::debug::random_sdp
// Provides: {"RandomSDP"}
// Dependencies: {}
# [doc = " A random, well-conditioned, symmetric definite-positive matrix."] # [derive (Clone , Debug)] pub struct RandomSDP < T : Scalar , D : Dim = Dyn > where DefaultAllocator : Allocator < D , D > , { m : OMatrix < T , D , D > , }
};
}
