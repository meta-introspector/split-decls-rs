// Generated macro for SignerInfoBuilder (struct)
macro_rules! Depcrate_builderSignerInfoBuilder {
() => {
// Module: crate::builder
// Provides: {"SignerInfoBuilder"}
// Dependencies: {}
# [doc = " Collect info needed for creating a `SignerInfo`."] # [doc = " Calling `build()` on this struct will"] # [doc = " - calculate the correct `CMSVersion` (depends on `sid`)"] # [doc = " - calculate the signature"] # [doc = " - set the signing time attribute"] # [doc = " - create a `SignerInfo` object"] pub struct SignerInfoBuilder < 's > { sid : SignerIdentifier , digest_algorithm : AlgorithmIdentifierOwned , signed_attributes : Option < Vec < Attribute > > , unsigned_attributes : Option < Vec < Attribute > > , encapsulated_content_info : & 's EncapsulatedContentInfo , external_message_digest : Option < & 's [u8] > , }
};
}
