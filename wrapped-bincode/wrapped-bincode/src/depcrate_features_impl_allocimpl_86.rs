// Generated macro for impl_86 (impl)
macro_rules! Depcrate_features_impl_allocimpl_86 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_86"}
// Dependencies: {}
impl < T > Encode for Rc < T > where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
};
}
