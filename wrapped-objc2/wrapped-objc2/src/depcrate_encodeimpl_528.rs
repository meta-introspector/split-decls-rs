// Generated macro for impl_528 (impl)
macro_rules! Depcrate_encodeimpl_528 {
() => {
// Module: crate::encode
// Provides: {"impl_528"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] unsafe impl < T : RefEncode > RefEncode for atomic :: AtomicPtr < T > { const ENCODING_REF : Encoding = Encoding :: Pointer (& Self :: ENCODING) ; }
};
}
