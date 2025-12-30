// Generated macro for has_doc (function)
macro_rules! Depcrate_builtinhas_doc {
() => {
// Module: crate::builtin
// Provides: {"has_doc"}
// Dependencies: {}
fn has_doc (attr : & hir :: Attribute) -> bool { if attr . is_doc_comment () { return true ; } if ! attr . has_name (sym :: doc) { return false ; } if attr . value_str () . is_some () { return true ; } if let Some (list) = attr . meta_item_list () { for meta in list { if meta . has_name (sym :: hidden) { return true ; } } } false }
};
}
