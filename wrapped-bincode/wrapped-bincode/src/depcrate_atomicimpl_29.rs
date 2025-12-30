// Generated macro for impl_29 (impl)
macro_rules! Depcrate_atomicimpl_29 {
() => {
// Module: crate::atomic
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (target_has_atomic = "8")] impl Encode for AtomicI8 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
