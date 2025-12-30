// Generated macro for impl_23 (impl)
macro_rules! Depcrate_atomicimpl_23 {
() => {
// Module: crate::atomic
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (target_has_atomic = "64")] impl Encode for AtomicU64 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
