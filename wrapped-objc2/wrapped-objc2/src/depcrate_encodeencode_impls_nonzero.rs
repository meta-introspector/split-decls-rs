// Generated macro for encode_impls_nonzero (macro)
macro_rules! Depcrate_encodeencode_impls_nonzero {
() => {
// Module: crate::encode
// Provides: {"encode_impls_nonzero"}
// Dependencies: {}
# [doc = " Helper for implementing [`Encode`] for nonzero integer types."] macro_rules ! encode_impls_nonzero { ($ ($ nonzero : ident => $ type : ty ,) *) => ($ (unsafe impl Encode for $ nonzero { const ENCODING : Encoding = <$ type >:: ENCODING ; } unsafe impl RefEncode for $ nonzero { const ENCODING_REF : Encoding = <$ type >:: ENCODING_REF ; } unsafe impl OptionEncode for $ nonzero { }) *) ; }
};
}
