// Generated macro for HashIgnoredAttrId (struct)
macro_rules! Depcrate_hirHashIgnoredAttrId {
() => {
// Module: crate::hir
// Provides: {"HashIgnoredAttrId"}
// Dependencies: {}
# [doc = " The derived implementation of [`HashStable_Generic`] on [`Attribute`]s shouldn't hash"] # [doc = " [`AttrId`]s. By wrapping them in this, we make sure we never do."] # [derive (Copy , Debug , Encodable , Decodable , Clone)] pub struct HashIgnoredAttrId { pub attr_id : AttrId , }
};
}
