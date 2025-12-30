// Generated macro for impl_527 (impl)
macro_rules! Depcrate_encodeimpl_527 {
() => {
// Module: crate::encode
// Provides: {"impl_527"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] unsafe impl < T : RefEncode > Encode for atomic :: AtomicPtr < T > { const ENCODING : Encoding = Encoding :: Atomic (& T :: ENCODING_REF) ; }
};
}
