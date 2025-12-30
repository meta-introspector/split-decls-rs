// Generated macro for impl_386 (impl)
macro_rules! Depcrate_de_implsimpl_386 {
() => {
// Module: crate::de::impls
// Provides: {"impl_386"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroUsize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroUsize :: new (usize :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: Usize , }) } }
};
}
