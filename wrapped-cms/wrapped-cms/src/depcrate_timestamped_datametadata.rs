// Generated macro for MetaData (struct)
macro_rules! Depcrate_timestamped_dataMetaData {
() => {
// Module: crate::timestamped_data
// Provides: {"MetaData"}
// Dependencies: {}
# [doc = " ```text"] # [doc = "  MetaData ::= SEQUENCE {"] # [doc = "     hashProtected        BOOLEAN,"] # [doc = "     fileName             UTF8String OPTIONAL,"] # [doc = "     mediaType            IA5String OPTIONAL,"] # [doc = "     otherMetaData        Attributes OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct MetaData { pub hash_protected : bool , # [asn1 (optional = "true")] pub file_name : Option < String > , # [asn1 (optional = "true")] pub media_type : Option < Ia5String > , # [asn1 (optional = "true")] pub other_meta_data : Option < Attributes > , }
};
}
