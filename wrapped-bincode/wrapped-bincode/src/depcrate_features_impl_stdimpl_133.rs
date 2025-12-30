// Generated macro for impl_133 (impl)
macro_rules! Depcrate_features_impl_stdimpl_133 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_133"}
// Dependencies: {}
impl < Context > Decode < Context > for Ipv4Addr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let mut buff = [0u8 ; 4] ; decoder . reader () . read (& mut buff) ? ; Ok (Self :: from (buff)) } }
};
}
