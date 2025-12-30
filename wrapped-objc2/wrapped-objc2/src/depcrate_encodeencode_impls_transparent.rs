// Generated macro for encode_impls_transparent (macro)
macro_rules! Depcrate_encodeencode_impls_transparent {
() => {
// Module: crate::encode
// Provides: {"encode_impls_transparent"}
// Dependencies: {}
macro_rules ! encode_impls_transparent { ($ ($ t : ident < T $ (: ?$ b : ident) ?>,) *) => ($ (unsafe impl < T : Encode $ (+ ?$ b) ?> Encode for $ t < T > { const ENCODING : Encoding = T :: ENCODING ; } unsafe impl < T : RefEncode $ (+ ?$ b) ?> RefEncode for $ t < T > { const ENCODING_REF : Encoding = T :: ENCODING_REF ; }) *) ; }
};
}
