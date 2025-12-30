// Generated macro for impl_141 (impl)
macro_rules! Depcrate_features_impl_stdimpl_141 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_141"}
// Dependencies: {}
impl Encode for SocketAddrV4 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . ip () . encode (encoder) ? ; self . port () . encode (encoder) } }
};
}
