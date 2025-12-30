// Generated macro for impl_11 (impl)
macro_rules! Depcrate_atomicimpl_11 {
() => {
// Module: crate::atomic
// Provides: {"impl_11"}
// Dependencies: {}
# [cfg (target_has_atomic = "8")] impl Encode for AtomicBool { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
