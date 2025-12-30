// Generated macro for encode_atomic_impls (macro)
macro_rules! Depcrate_encodeencode_atomic_impls {
() => {
// Module: crate::encode
// Provides: {"encode_atomic_impls"}
// Dependencies: {}
# [doc = " Helper for implementing for atomic types."] macro_rules ! encode_atomic_impls { ($ ($ (# [$ m : meta]) * $ atomic : ident => $ type : ty ,) *) => ($ ($ (# [$ m]) * unsafe impl Encode for atomic ::$ atomic { const ENCODING : Encoding = Encoding :: Atomic (&<$ type >:: ENCODING) ; } $ (# [$ m]) * unsafe impl RefEncode for atomic ::$ atomic { const ENCODING_REF : Encoding = Encoding :: Pointer (& Self :: ENCODING) ; }) *) ; }
};
}
