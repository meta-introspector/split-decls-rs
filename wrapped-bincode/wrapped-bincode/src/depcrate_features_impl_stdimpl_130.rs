// Generated macro for impl_130 (impl)
macro_rules! Depcrate_features_impl_stdimpl_130 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_130"}
// Dependencies: {}
impl < Context > Decode < Context > for IpAddr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u32 :: decode (decoder) ? { 0 => Ok (IpAddr :: V4 (Ipv4Addr :: decode (decoder) ?)) , 1 => Ok (IpAddr :: V6 (Ipv6Addr :: decode (decoder) ?)) , found => Err (DecodeError :: UnexpectedVariant { allowed : & crate :: error :: AllowedEnumVariants :: Range { min : 0 , max : 1 } , found , type_name : core :: any :: type_name :: < IpAddr > () , }) , } } }
};
}
