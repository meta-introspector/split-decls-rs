// Generated macro for impl_531 (impl)
macro_rules! Depcrate_encodeimpl_531 {
() => {
// Module: crate::encode
// Provides: {"impl_531"}
// Dependencies: {}
unsafe impl < T : Encode , const LENGTH : usize > RefEncode for [T ; LENGTH] { const ENCODING_REF : Encoding = Encoding :: Pointer (& Self :: ENCODING) ; }
};
}
