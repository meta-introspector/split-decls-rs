// Generated macro for impl_442 (impl)
macro_rules! Depcrate_de_implsimpl_442 {
() => {
// Module: crate::de::impls
// Provides: {"impl_442"}
// Dependencies: {}
impl < Context , T > Decode < Context > for RangeInclusive < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let min = T :: decode (decoder) ? ; let max = T :: decode (decoder) ? ; Ok (RangeInclusive :: new (min , max)) } }
};
}
