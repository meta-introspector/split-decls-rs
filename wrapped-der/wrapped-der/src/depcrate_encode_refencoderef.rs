// Generated macro for EncodeRef (struct)
macro_rules! Depcrate_encode_refEncodeRef {
() => {
// Module: crate::encode_ref
// Provides: {"EncodeRef"}
// Dependencies: {}
# [doc = " Reference encoder: wrapper type which impls `Encode` for any reference to a"] # [doc = " type which impls the same."] pub struct EncodeRef < 'a , T > (pub & 'a T) ;
};
}
