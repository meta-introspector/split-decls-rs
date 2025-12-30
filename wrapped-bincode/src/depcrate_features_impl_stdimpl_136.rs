// Generated macro for impl_136 (impl)
macro_rules! Depcrate_features_impl_stdimpl_136 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_136"}
// Dependencies: {}
impl < Context > Decode < Context > for Ipv6Addr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let mut buff = [0u8 ; 16] ; decoder . reader () . read (& mut buff) ? ; Ok (Self :: from (buff)) } }
};
}
