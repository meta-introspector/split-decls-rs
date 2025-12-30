// Generated macro for impl_41 (impl)
macro_rules! Depcrate_atomicimpl_41 {
() => {
// Module: crate::atomic
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl Encode for AtomicIsize { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
};
}
