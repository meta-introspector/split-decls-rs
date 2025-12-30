// Generated macro for impl_17 (impl)
macro_rules! Depcrate_atomicimpl_17 {
() => {
// Module: crate::atomic
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (target_has_atomic = "16")] impl Encode for AtomicU16 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
