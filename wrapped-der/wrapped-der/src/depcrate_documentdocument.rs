// Generated macro for Document (struct)
macro_rules! Depcrate_documentDocument {
() => {
// Module: crate::document
// Provides: {"Document"}
// Dependencies: {}
# [doc = " ASN.1 DER-encoded document."] # [doc = ""] # [doc = " This type wraps an encoded ASN.1 DER message. The document checked to"] # [doc = " ensure it contains a valid DER-encoded `SEQUENCE`."] # [doc = ""] # [doc = " It implements common functionality related to encoding/decoding such"] # [doc = " documents, such as PEM encapsulation as well as reading/writing documents"] # [doc = " from/to the filesystem."] # [doc = ""] # [doc = " The [`SecretDocument`] provides a wrapper for this type with additional"] # [doc = " hardening applied."] # [derive (Clone , Eq , PartialEq)] pub struct Document { # [doc = " ASN.1 DER encoded bytes."] der_bytes : Vec < u8 > , # [doc = " Length of this document."] length : Length , }
};
}
