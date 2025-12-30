// Generated macro for impl_14 (impl)
macro_rules! Depcrate_atomicimpl_14 {
() => {
// Module: crate::atomic
// Provides: {"impl_14"}
// Dependencies: {}
# [cfg (target_has_atomic = "8")] impl Encode for AtomicU8 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
