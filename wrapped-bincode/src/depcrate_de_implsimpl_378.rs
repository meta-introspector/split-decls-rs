// Generated macro for impl_378 (impl)
macro_rules! Depcrate_de_implsimpl_378 {
() => {
// Module: crate::de::impls
// Provides: {"impl_378"}
// Dependencies: {}
impl < Context > Decode < Context > for NonZeroU64 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU64 :: new (u64 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U64 , }) } }
};
}
