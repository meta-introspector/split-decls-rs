// Generated macro for impl_402 (impl)
macro_rules! Depcrate_de_implsimpl_402 {
() => {
// Module: crate::de::impls
// Provides: {"impl_402"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroI64 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI64 :: new (i64 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I64 , }) } }
};
}
