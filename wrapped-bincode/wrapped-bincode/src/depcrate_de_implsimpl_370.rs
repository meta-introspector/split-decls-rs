// Generated macro for impl_370 (impl)
macro_rules! Depcrate_de_implsimpl_370 {
() => {
// Module: crate::de::impls
// Provides: {"impl_370"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroU16 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU16 :: new (u16 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U16 , }) } }
};
}
