// Generated macro for apply_to_callsite (function)
macro_rules! Depcrate_attributesapply_to_callsite {
() => {
// Module: crate::attributes
// Provides: {"apply_to_callsite"}
// Dependencies: {}
pub (crate) fn apply_to_callsite (callsite : & Value , idx : AttributePlace , attrs : & [& Attribute]) { if ! attrs . is_empty () { llvm :: AddCallSiteAttributes (callsite , idx , attrs) ; } }
};
}
