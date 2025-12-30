// Generated macro for impl_366 (impl)
macro_rules! Depcrate_de_implsimpl_366 {
() => {
// Module: crate::de::impls
// Provides: {"impl_366"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroU8 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU8 :: new (u8 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U8 , }) } }
};
}
