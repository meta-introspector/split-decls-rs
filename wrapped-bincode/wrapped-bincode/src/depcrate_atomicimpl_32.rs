// Generated macro for impl_32 (impl)
macro_rules! Depcrate_atomicimpl_32 {
() => {
// Module: crate::atomic
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (target_has_atomic = "16")] impl Encode for AtomicI16 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
