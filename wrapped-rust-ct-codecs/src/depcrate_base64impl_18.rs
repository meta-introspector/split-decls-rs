// Generated macro for impl_18 (impl)
macro_rules! Depcrate_base64impl_18 {
() => {
// Module: crate::base64
// Provides: {"impl_18"}
// Dependencies: {}
impl Encoder for Base64UrlSafeNoPadding { # [inline] fn encoded_len (bin_len : usize) -> Result < usize , Error > { Base64Impl :: encoded_len (bin_len , Base64Variant :: UrlSafeNoPadding) } # [inline] fn encode < IN : AsRef < [u8] > > (b64 : & mut [u8] , bin : IN) -> Result < & [u8] , Error > { Base64Impl :: encode (b64 , bin . as_ref () , Base64Variant :: UrlSafeNoPadding) } }
};
}
