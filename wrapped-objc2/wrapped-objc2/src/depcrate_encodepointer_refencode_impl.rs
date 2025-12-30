// Generated macro for pointer_refencode_impl (macro)
macro_rules! Depcrate_encodepointer_refencode_impl {
() => {
// Module: crate::encode
// Provides: {"pointer_refencode_impl"}
// Dependencies: {}
# [doc = " Helper for implementing [`RefEncode`]."] macro_rules ! pointer_refencode_impl { ($ ($ t : ty) ,*) => ($ (unsafe impl RefEncode for $ t { const ENCODING_REF : Encoding = Encoding :: Pointer (& Self :: ENCODING) ; }) *) ; }
};
}
