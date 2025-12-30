// Generated macro for impl_38 (impl)
macro_rules! Depcrate_atomicimpl_38 {
() => {
// Module: crate::atomic
// Provides: {"impl_38"}
// Dependencies: {}
# [cfg (target_has_atomic = "64")] impl Encode for AtomicI64 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
