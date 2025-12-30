// Generated macro for find_doc_hint (function)
macro_rules! Depcrate_attrsfind_doc_hint {
() => {
// Module: crate::attrs
// Provides: {"find_doc_hint"}
// Dependencies: {}
# [doc = " Returns the value of the first `doc_hint` attribute in the given slice or"] # [doc = " `None` if `doc_hint` attribute is not available."] pub fn find_doc_hint (attrs : & [syn :: Attribute]) -> Option < String > { attrs . iter () . filter_map (doc_hint) . next () }
};
}
