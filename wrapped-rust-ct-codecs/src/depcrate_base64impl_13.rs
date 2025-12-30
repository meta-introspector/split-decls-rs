// Generated macro for impl_13 (impl)
macro_rules! Depcrate_base64impl_13 {
() => {
// Module: crate::base64
// Provides: {"impl_13"}
// Dependencies: {}
impl Decoder for Base64 { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: Original) } }
};
}
