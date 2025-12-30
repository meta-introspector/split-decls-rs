// Generated macro for KeyAgreeRecipientInfo (struct)
macro_rules! Depcrate_enveloped_dataKeyAgreeRecipientInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"KeyAgreeRecipientInfo"}
// Dependencies: {}
# [doc = " The `KeyAgreeRecipientInfo` type is defined in [RFC 5652 Section 6.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   KeyAgreeRecipientInfo ::= SEQUENCE {"] # [doc = "       version CMSVersion,  -- always set to 3"] # [doc = "       originator [0] EXPLICIT OriginatorIdentifierOrKey,"] # [doc = "       ukm [1] EXPLICIT UserKeyingMaterial OPTIONAL,"] # [doc = "       keyEncryptionAlgorithm AlgorithmIdentifier"] # [doc = "           {KEY-AGREE, {KeyAgreementAlgorithmSet}},"] # [doc = "       recipientEncryptedKeys RecipientEncryptedKeys }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct KeyAgreeRecipientInfo { pub version : CmsVersion , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT")] pub originator : OriginatorIdentifierOrKey , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , optional = "true")] pub ukm : Option < UserKeyingMaterial > , pub key_enc_alg : AlgorithmIdentifierOwned , pub recipient_enc_keys : RecipientEncryptedKeys , }
};
}
