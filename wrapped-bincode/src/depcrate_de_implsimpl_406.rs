// Generated macro for impl_406 (impl)
macro_rules! Depcrate_de_implsimpl_406 {
() => {
// Module: crate::de::impls
// Provides: {"impl_406"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroI128 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI128 :: new (i128 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I128 , }) } }
};
}
