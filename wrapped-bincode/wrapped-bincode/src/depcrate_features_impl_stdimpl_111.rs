// Generated macro for impl_111 (impl)
macro_rules! Depcrate_features_impl_stdimpl_111 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_111"}
// Dependencies: {}
impl Encode for & CStr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . to_bytes () . encode (encoder) } }
};
}
