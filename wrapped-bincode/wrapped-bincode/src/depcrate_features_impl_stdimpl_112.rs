// Generated macro for impl_112 (impl)
macro_rules! Depcrate_features_impl_stdimpl_112 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_112"}
// Dependencies: {}
impl Encode for CString { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_bytes () . encode (encoder) } }
};
}
