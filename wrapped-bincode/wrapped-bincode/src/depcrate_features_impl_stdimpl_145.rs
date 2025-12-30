// Generated macro for impl_145 (impl)
macro_rules! Depcrate_features_impl_stdimpl_145 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_145"}
// Dependencies: {}
impl < Context > Decode < Context > for SocketAddrV6 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let ip = Ipv6Addr :: decode (decoder) ? ; let port = u16 :: decode (decoder) ? ; Ok (Self :: new (ip , port , 0 , 0)) } }
};
}
