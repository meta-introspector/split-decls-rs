// Generated macro for impl_132 (impl)
macro_rules! Depcrate_features_impl_stdimpl_132 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_132"}
// Dependencies: {}
impl Encode for Ipv4Addr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& self . octets ()) } }
};
}
