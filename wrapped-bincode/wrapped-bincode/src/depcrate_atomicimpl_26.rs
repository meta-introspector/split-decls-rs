// Generated macro for impl_26 (impl)
macro_rules! Depcrate_atomicimpl_26 {
() => {
// Module: crate::atomic
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl Encode for AtomicUsize { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
