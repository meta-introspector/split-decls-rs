// Generated macro for apply_to_llfn (function)
macro_rules! Depcrate_attributesapply_to_llfn {
() => {
// Module: crate::attributes
// Provides: {"apply_to_llfn"}
// Dependencies: {}
pub (crate) fn apply_to_llfn (llfn : & Value , idx : AttributePlace , attrs : & [& Attribute]) { if ! attrs . is_empty () { llvm :: AddFunctionAttributes (llfn , idx , attrs) ; } }
};
}
