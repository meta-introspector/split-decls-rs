// Generated macro for Asn1Type (enum)
macro_rules! Depcrate_asn1_typeAsn1Type {
() => {
// Module: crate::asn1_type
// Provides: {"Asn1Type"}
// Dependencies: {}
# [doc = " ASN.1 built-in types supported by the `#[asn1(type = \"...\")]` attribute"] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub (crate) enum Asn1Type { # [doc = " ASN.1 `BIT STRING`."] BitString , # [doc = " ASN.1 `IA5String`."] Ia5String , # [doc = " ASN.1 `GeneralizedTime`."] GeneralizedTime , # [doc = " ASN.1 `OCTET STRING`."] OctetString , # [doc = " ASN.1 `PrintableString`."] PrintableString , # [doc = " ASN.1 `TeletexString`."] TeletexString , # [doc = " ASN.1 `VideotexString`."] VideotexString , # [doc = " ASN.1 `UTCTime`."] UtcTime , # [doc = " ASN.1 `UTF8String`."] Utf8String , # [doc = " ASN.1 `BMPString`."] BmpString , }
};
}
