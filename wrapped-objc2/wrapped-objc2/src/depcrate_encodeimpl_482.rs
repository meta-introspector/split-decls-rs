// Generated macro for impl_482 (impl)
macro_rules! Depcrate_encodeimpl_482 {
() => {
// Module: crate::encode
// Provides: {"impl_482"}
// Dependencies: {}
unsafe impl < T : Encode + OptionEncode > Encode for Option < T > { const ENCODING : Encoding = { if mem :: size_of :: < T > () != mem :: size_of :: < Option < T > > () { panic ! ("invalid OptionEncode + Encode implementation") ; } T :: ENCODING } ; }
};
}
