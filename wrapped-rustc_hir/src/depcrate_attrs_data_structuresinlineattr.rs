// Generated macro for InlineAttr (enum)
macro_rules! Depcrate_attrs_data_structuresInlineAttr {
() => {
// Module: crate::attrs::data_structures
// Provides: {"InlineAttr"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Encodable , Decodable , Debug , HashStable_Generic , PrintAttribute)] pub enum InlineAttr { None , Hint , Always , Never , # [doc = " `#[rustc_force_inline]` forces inlining to happen in the MIR inliner - it reports an error"] # [doc = " if the inlining cannot happen. It is limited to only free functions so that the calls"] # [doc = " can always be resolved."] Force { attr_span : Span , reason : Option < Symbol > , } , }
};
}
