// Generated macro for EncryptionInfo (struct)
macro_rules! Depcrate_timestamped_dataEncryptionInfo {
() => {
// Module: crate::timestamped_data
// Provides: {"EncryptionInfo"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " EncryptionInfo ::= SEQUENCE {"] # [doc = "     encryptionInfoType     OBJECT IDENTIFIER,"] # [doc = "     encryptionInfoValue    ANY DEFINED BY encryptionInfoType"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EncryptionInfo { pub encryption_info_type : ObjectIdentifier , pub encryption_info_value : Any , }
};
}
