// Generated macro for impl_15 (impl)
macro_rules! Depcrate_base64impl_15 {
() => {
// Module: crate::base64
// Provides: {"impl_15"}
// Dependencies: {}
impl Decoder for Base64NoPadding { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: OriginalNoPadding) } }
};
}
