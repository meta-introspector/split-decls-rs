// Generated macro for impl_135 (impl)
macro_rules! Depcrate_features_impl_stdimpl_135 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_135"}
// Dependencies: {}
impl Encode for Ipv6Addr { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& self . octets ()) } }
};
}
