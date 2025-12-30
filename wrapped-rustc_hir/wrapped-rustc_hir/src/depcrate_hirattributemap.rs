// Generated macro for AttributeMap (struct)
macro_rules! Depcrate_hirAttributeMap {
() => {
// Module: crate::hir
// Provides: {"AttributeMap"}
// Dependencies: {}
# [doc = " Attributes owned by a HIR owner."] # [derive (Debug)] pub struct AttributeMap < 'tcx > { pub map : SortedMap < ItemLocalId , & 'tcx [Attribute] > , # [doc = " Preprocessed `#[define_opaque]` attribute."] pub define_opaque : Option < & 'tcx [(Span , LocalDefId)] > , pub opt_hash : Option < Fingerprint > , }
};
}
