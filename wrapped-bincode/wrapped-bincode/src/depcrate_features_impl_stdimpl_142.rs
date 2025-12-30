// Generated macro for impl_142 (impl)
macro_rules! Depcrate_features_impl_stdimpl_142 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_142"}
// Dependencies: {}
impl < Context > Decode < Context > for SocketAddrV4 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let ip = Ipv4Addr :: decode (decoder) ? ; let port = u16 :: decode (decoder) ? ; Ok (Self :: new (ip , port)) } }
};
}
