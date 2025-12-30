// Generated macro for impl_93 (impl)
macro_rules! Depcrate_features_impl_allocimpl_93 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_93"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < T > Encode for Arc < T > where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
};
}
