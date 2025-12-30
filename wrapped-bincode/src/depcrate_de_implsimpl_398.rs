// Generated macro for impl_398 (impl)
macro_rules! Depcrate_de_implsimpl_398 {
() => {
// Module: crate::de::impls
// Provides: {"impl_398"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroI32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI32 :: new (i32 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I32 , }) } }
};
}
