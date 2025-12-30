// Generated macro for impl_99 (impl)
macro_rules! Depcrate_builderimpl_99 {
() => {
// Module: crate::builder
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'c , R > EnvelopedDataBuilder < 'c , R > { # [doc = " Create a new builder for `EnvelopedData`"] pub fn new (originator_info : Option < OriginatorInfo > , unencrypted_content : & 'c [u8] , content_encryption_algorithm : ContentEncryptionAlgorithm , unprotected_attributes : Option < Attributes > ,) -> Result < Self > { Ok (EnvelopedDataBuilder { originator_info , recipient_infos : Vec :: new () , unencrypted_content , content_encryption_algorithm , unprotected_attributes , }) } }
};
}
