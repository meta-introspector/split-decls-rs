// Generated macro for impl_72 (impl)
macro_rules! Depcrate_features_impl_allocimpl_72 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_72"}
// Dependencies: {}
impl Encode for String { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_bytes () . encode (encoder) } }
};
}
