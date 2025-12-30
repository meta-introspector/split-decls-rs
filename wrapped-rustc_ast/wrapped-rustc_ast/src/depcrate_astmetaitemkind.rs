// Generated macro for MetaItemKind (enum)
macro_rules! Depcrate_astMetaItemKind {
() => {
// Module: crate::ast
// Provides: {"MetaItemKind"}
// Dependencies: {}
# [doc = " The meta item kind, containing the data after the initial path."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic)] pub enum MetaItemKind { # [doc = " Word meta item."] # [doc = ""] # [doc = " E.g., `#[test]`, which lacks any arguments after `test`."] Word , # [doc = " List meta item."] # [doc = ""] # [doc = " E.g., `#[derive(..)]`, where the field represents the `..`."] List (ThinVec < MetaItemInner >) , # [doc = " Name value meta item."] # [doc = ""] # [doc = " E.g., `#[feature = \"foo\"]`, where the field represents the `\"foo\"`."] NameValue (MetaItemLit) , }
};
}
