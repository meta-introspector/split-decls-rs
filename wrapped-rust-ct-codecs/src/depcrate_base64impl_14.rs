// Generated macro for impl_14 (impl)
macro_rules! Depcrate_base64impl_14 {
() => {
// Module: crate::base64
// Provides: {"impl_14"}
// Dependencies: {}
impl Encoder for Base64NoPadding { # [inline] fn encoded_len (bin_len : usize) -> Result < usize , Error > { Base64Impl :: encoded_len (bin_len , Base64Variant :: OriginalNoPadding) } # [inline] fn encode < IN : AsRef < [u8] > > (b64 : & mut [u8] , bin : IN) -> Result < & [u8] , Error > { Base64Impl :: encode (b64 , bin . as_ref () , Base64Variant :: OriginalNoPadding) } }
};
}
