// Generated macro for impl_35 (impl)
macro_rules! Depcrate_atomicimpl_35 {
() => {
// Module: crate::atomic
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg (target_has_atomic = "32")] impl Encode for AtomicI32 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
