// Generated macro for OtherRevocationInfoFormat (struct)
macro_rules! Depcrate_revocationOtherRevocationInfoFormat {
() => {
// Module: crate::revocation
// Provides: {"OtherRevocationInfoFormat"}
// Dependencies: {}
# [doc = " The `RevocationInfoChoices` type is defined in [RFC 5652 Section 10.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OtherRevocationInfoFormat ::= SEQUENCE {"] # [doc = "       otherRevInfoFormat    OTHER-REVOK-INFO."] # [doc = "               &id({SupportedOtherRevokInfo}),"] # [doc = "       otherRevInfo          OTHER-REVOK-INFO."] # [doc = "               &Type({SupportedOtherRevokInfo}{@otherRevInfoFormat})}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.1]: https://www.rfc-editor.org/rfc/rfc5652#section-10.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OtherRevocationInfoFormat { pub other_format : AlgorithmIdentifierOwned , pub other : Any , }
};
}
