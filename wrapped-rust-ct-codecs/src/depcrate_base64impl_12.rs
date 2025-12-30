// Generated macro for impl_12 (impl)
macro_rules! Depcrate_base64impl_12 {
() => {
// Module: crate::base64
// Provides: {"impl_12"}
// Dependencies: {}
impl Encoder for Base64 { # [inline] fn encoded_len (bin_len : usize) -> Result < usize , Error > { Base64Impl :: encoded_len (bin_len , Base64Variant :: Original) } # [inline] fn encode < IN : AsRef < [u8] > > (b64 : & mut [u8] , bin : IN) -> Result < & [u8] , Error > { Base64Impl :: encode (b64 , bin . as_ref () , Base64Variant :: Original) } }
};
}
