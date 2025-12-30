// Generated macro for impl_20 (impl)
macro_rules! Depcrate_atomicimpl_20 {
() => {
// Module: crate::atomic
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (target_has_atomic = "32")] impl Encode for AtomicU32 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
