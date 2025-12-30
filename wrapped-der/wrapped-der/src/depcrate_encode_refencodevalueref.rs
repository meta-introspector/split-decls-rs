// Generated macro for EncodeValueRef (struct)
macro_rules! Depcrate_encode_refEncodeValueRef {
() => {
// Module: crate::encode_ref
// Provides: {"EncodeValueRef"}
// Dependencies: {}
# [doc = " Reference value encoder: wrapper type which impls `EncodeValue` and `Tagged`"] # [doc = " for any reference type which impls the same."] # [doc = ""] # [doc = " By virtue of the blanket impl, this type also impls `Encode`."] pub struct EncodeValueRef < 'a , T > (pub & 'a T) ;
};
}
