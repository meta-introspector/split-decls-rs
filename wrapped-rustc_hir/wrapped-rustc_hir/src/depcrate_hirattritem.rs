// Generated macro for AttrItem (struct)
macro_rules! Depcrate_hirAttrItem {
() => {
// Module: crate::hir
// Provides: {"AttrItem"}
// Dependencies: {}
# [derive (Clone , Debug , HashStable_Generic , Encodable , Decodable)] pub struct AttrItem { pub path : AttrPath , pub args : AttrArgs , pub id : HashIgnoredAttrId , # [doc = " Denotes if the attribute decorates the following construct (outer)"] # [doc = " or the construct this attribute is contained within (inner)."] pub style : AttrStyle , # [doc = " Span of the entire attribute"] pub span : Span , }
};
}
