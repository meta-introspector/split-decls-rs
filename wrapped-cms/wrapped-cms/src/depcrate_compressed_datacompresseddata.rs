// Generated macro for CompressedData (struct)
macro_rules! Depcrate_compressed_dataCompressedData {
() => {
// Module: crate::compressed_data
// Provides: {"CompressedData"}
// Dependencies: {}
# [doc = " The `CompressedData` type is defined in [RFC 3274 Section 1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " CompressedData ::= SEQUENCE {"] # [doc = "     version CMSVersion,"] # [doc = "     compressionAlgorithm CompressionAlgorithmIdentifier,"] # [doc = "     encapContentInfo EncapsulatedContentInfo"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 3274 Section 1.1]: https://www.rfc-editor.org/rfc/rfc3274#section-1.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CompressedData { pub version : CmsVersion , pub compression_alg : AlgorithmIdentifierOwned , pub encap_content_info : EncapsulatedContentInfo , }
};
}
