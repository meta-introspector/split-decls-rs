// Generated macro for EnvelopedDataBuilder (struct)
macro_rules! Depcrate_builderEnvelopedDataBuilder {
() => {
// Module: crate::builder
// Provides: {"EnvelopedDataBuilder"}
// Dependencies: {}
# [doc = " Builds CMS `EnvelopedData` according to RFC 5652 § 6."] pub struct EnvelopedDataBuilder < 'c , R : ? Sized > { originator_info : Option < OriginatorInfo > , recipient_infos : Vec < Box < dyn RecipientInfoBuilder < Rng = R > + 'c > > , unencrypted_content : & 'c [u8] , content_encryption_algorithm : ContentEncryptionAlgorithm , unprotected_attributes : Option < Attributes > , }
};
}
