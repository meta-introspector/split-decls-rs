// Generated macro for impl_374 (impl)
macro_rules! Depcrate_de_implsimpl_374 {
() => {
// Module: crate::de::impls
// Provides: {"impl_374"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroU32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU32 :: new (u32 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U32 , }) } }
};
}
