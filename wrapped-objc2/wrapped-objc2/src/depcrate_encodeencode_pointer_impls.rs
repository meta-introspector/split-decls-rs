// Generated macro for encode_pointer_impls (macro)
macro_rules! Depcrate_encodeencode_pointer_impls {
() => {
// Module: crate::encode
// Provides: {"encode_pointer_impls"}
// Dependencies: {}
# [doc = " Helper for implementing `Encode`/`RefEncode` for pointers to types that"] # [doc = " implement `RefEncode`."] # [doc = ""] # [doc = " Using `?Sized` is safe here because we delegate to other implementations"] # [doc = " (which will verify that the implementation is safe for the unsized type)."] macro_rules ! encode_pointer_impls { (unsafe impl < T : RefEncode > $ x : ident for Pointer < T > { const $ c : ident = $ e : expr ; }) => (unsafe impl < T : RefEncode + ? Sized > $ x for * const T { const $ c : Encoding = $ e ; } unsafe impl < T : RefEncode + ? Sized > $ x for * mut T { const $ c : Encoding = $ e ; } unsafe impl <'a , T : RefEncode + ? Sized > $ x for &'a T { const $ c : Encoding = $ e ; } unsafe impl <'a , T : RefEncode + ? Sized > $ x for &'a mut T { const $ c : Encoding = $ e ; } unsafe impl < T : RefEncode + ? Sized > $ x for NonNull < T > { const $ c : Encoding = $ e ; }) ; }
};
}
