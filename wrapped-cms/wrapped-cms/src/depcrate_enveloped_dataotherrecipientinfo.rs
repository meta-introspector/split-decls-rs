// Generated macro for OtherRecipientInfo (struct)
macro_rules! Depcrate_enveloped_dataOtherRecipientInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"OtherRecipientInfo"}
// Dependencies: {}
# [doc = " The `OtherRecipientInfo` type is defined in [RFC 5652 Section 6.2.5]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OtherRecipientInfo ::= SEQUENCE {"] # [doc = "       oriType    OTHER-RECIPIENT."] # [doc = "               &id({SupportedOtherRecipInfo}),"] # [doc = "       oriValue   OTHER-RECIPIENT."] # [doc = "               &Type({SupportedOtherRecipInfo}{@oriType})}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.5]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.5"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OtherRecipientInfo { pub ori_type : ObjectIdentifier , pub ori_value : Any , }
};
}
