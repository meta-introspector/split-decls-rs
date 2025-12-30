// Generated macro for impl_139 (impl)
macro_rules! Depcrate_features_impl_stdimpl_139 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_139"}
// Dependencies: {}
impl < Context > Decode < Context > for SocketAddr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u32 :: decode (decoder) ? { 0 => Ok (SocketAddr :: V4 (SocketAddrV4 :: decode (decoder) ?)) , 1 => Ok (SocketAddr :: V6 (SocketAddrV6 :: decode (decoder) ?)) , found => Err (DecodeError :: UnexpectedVariant { allowed : & crate :: error :: AllowedEnumVariants :: Range { min : 0 , max : 1 } , found , type_name : core :: any :: type_name :: < SocketAddr > () , }) , } } }
};
}
