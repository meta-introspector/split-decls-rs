// Generated macro for impl_390 (impl)
macro_rules! Depcrate_de_implsimpl_390 {
() => {
// Module: crate::de::impls
// Provides: {"impl_390"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroI8 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI8 :: new (i8 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I8 , }) } }
};
}
