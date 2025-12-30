// Generated macro for impl_17 (impl)
macro_rules! Depcrate_base64impl_17 {
() => {
// Module: crate::base64
// Provides: {"impl_17"}
// Dependencies: {}
impl Decoder for Base64UrlSafe { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: UrlSafe) } }
};
}
