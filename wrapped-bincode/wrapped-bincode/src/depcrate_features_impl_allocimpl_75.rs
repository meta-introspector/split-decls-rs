// Generated macro for impl_75 (impl)
macro_rules! Depcrate_features_impl_allocimpl_75 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_75"}
// Dependencies: {}
impl < T > Encode for Box < T > where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
};
}
