// Generated macro for encode_impls_size (macro)
macro_rules! Depcrate_encodeencode_impls_size {
() => {
// Module: crate::encode
// Provides: {"encode_impls_size"}
// Dependencies: {}
macro_rules ! encode_impls_size { ($ ($ t : ty => ($ t16 : ty , $ t32 : ty , $ t64 : ty) ,) *) => ($ (# [doc = concat ! ("The encoding of [`" , stringify ! ($ t) , "`] varies based on the target pointer width.")] unsafe impl Encode for $ t { # [cfg (target_pointer_width = "16")] const ENCODING : Encoding = <$ t16 >:: ENCODING ; # [cfg (target_pointer_width = "32")] const ENCODING : Encoding = <$ t32 >:: ENCODING ; # [cfg (target_pointer_width = "64")] const ENCODING : Encoding = <$ t64 >:: ENCODING ; }) *) ; }
};
}
