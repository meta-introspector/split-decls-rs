// Generated macro for impl_394 (impl)
macro_rules! Depcrate_de_implsimpl_394 {
() => {
// Module: crate::de::impls
// Provides: {"impl_394"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroI16 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI16 :: new (i16 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I16 , }) } }
};
}
