// Generated macro for TimeStampedData (struct)
macro_rules! Depcrate_timestamped_dataTimeStampedData {
() => {
// Module: crate::timestamped_data
// Provides: {"TimeStampedData"}
// Dependencies: {}
# [doc = " The `TimeStampedData` type is defined in [RFC 5544 Section 2]."] # [doc = ""] # [doc = " ```text"] # [doc = " TimeStampedData ::= SEQUENCE {"] # [doc = "     version              INTEGER { v1(1) },"] # [doc = "     dataUri              IA5String OPTIONAL,"] # [doc = "     metaData             MetaData OPTIONAL,"] # [doc = "     content              OCTET STRING OPTIONAL,"] # [doc = "     temporalEvidence     Evidence"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5544 Section 2]: https://www.rfc-editor.org/rfc/rfc5544#section-2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct TimeStampedData < 'a > { pub version : TsdVersion , # [asn1 (optional = "true")] pub data_uri : Option < Ia5String > , # [asn1 (optional = "true")] pub meta_data : Option < MetaData > , # [asn1 (optional = "true")] pub content : Option < & 'a OctetStringRef > , pub temporal_evidence : Evidence , }
};
}
