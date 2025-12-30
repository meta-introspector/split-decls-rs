// Generated macro for impl_410 (impl)
macro_rules! Depcrate_de_implsimpl_410 {
() => {
// Module: crate::de::impls
// Provides: {"impl_410"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroIsize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroIsize :: new (isize :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: Isize , }) } }
};
}
