// Generated macro for impl_138 (impl)
macro_rules! Depcrate_features_impl_stdimpl_138 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_138"}
// Dependencies: {}
impl Encode for SocketAddr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { SocketAddr :: V4 (v4) => { 0u32 . encode (encoder) ? ; v4 . encode (encoder) } SocketAddr :: V6 (v6) => { 1u32 . encode (encoder) ? ; v6 . encode (encoder) } } } }
};
}
