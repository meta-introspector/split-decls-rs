// Generated macro for TokenType (enum)
macro_rules! Depcrate_content_yaml_vendored_scannerTokenType {
() => {
// Module: crate::content::yaml::vendored::scanner
// Provides: {"TokenType"}
// Dependencies: {}
# [derive (Clone , PartialEq , Debug , Eq)] pub enum TokenType { StreamStart (TEncoding) , StreamEnd , # [doc = " major, minor"] VersionDirective (u32 , u32) , # [doc = " handle, prefix"] TagDirective (String , String) , DocumentStart , DocumentEnd , BlockSequenceStart , BlockMappingStart , BlockEnd , FlowSequenceStart , FlowSequenceEnd , FlowMappingStart , FlowMappingEnd , BlockEntry , FlowEntry , Key , Value , Alias (String) , Anchor (String) , # [doc = " handle, suffix"] Tag (String , String) , Scalar (TScalarStyle , String) , }
};
}
