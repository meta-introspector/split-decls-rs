// Generated macro for encode_impls (macro)
macro_rules! Depcrate_encodeencode_impls {
() => {
// Module: crate::encode
// Provides: {"encode_impls"}
// Dependencies: {}
# [doc = " Helper for implementing [`Encode`]."] macro_rules ! encode_impls { ($ ($ t : ty => $ e : ident ,) *) => ($ (unsafe impl Encode for $ t { const ENCODING : Encoding = Encoding ::$ e ; }) *) ; }
};
}
