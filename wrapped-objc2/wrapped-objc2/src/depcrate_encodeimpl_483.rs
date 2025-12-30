// Generated macro for impl_483 (impl)
macro_rules! Depcrate_encodeimpl_483 {
() => {
// Module: crate::encode
// Provides: {"impl_483"}
// Dependencies: {}
unsafe impl < T : RefEncode + OptionEncode > RefEncode for Option < T > { const ENCODING_REF : Encoding = { if mem :: size_of :: < T > () != mem :: size_of :: < Option < T > > () { panic ! ("invalid OptionEncode + RefEncode implementation") ; } T :: ENCODING_REF } ; }
};
}
