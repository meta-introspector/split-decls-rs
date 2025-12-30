// Generated macro for impl_382 (impl)
macro_rules! Depcrate_de_implsimpl_382 {
() => {
// Module: crate::de::impls
// Provides: {"impl_382"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroU128 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU128 :: new (u128 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U128 , }) } }
};
}
