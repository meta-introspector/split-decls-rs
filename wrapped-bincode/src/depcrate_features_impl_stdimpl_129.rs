// Generated macro for impl_129 (impl)
macro_rules! Depcrate_features_impl_stdimpl_129 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_129"}
// Dependencies: {}
impl Encode for IpAddr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { IpAddr :: V4 (v4) => { 0u32 . encode (encoder) ? ; v4 . encode (encoder) } IpAddr :: V6 (v6) => { 1u32 . encode (encoder) ? ; v6 . encode (encoder) } } } }
};
}
