// Generated macro for resolve_doc_path_on (function)
macro_rules! Depcrate_attrsresolve_doc_path_on {
() => {
// Module: crate::attrs
// Provides: {"resolve_doc_path_on"}
// Dependencies: {}
# [doc = " Resolves the item `link` points to in the scope of `def`."] pub fn resolve_doc_path_on (db : & dyn HirDatabase , def : impl HasAttrs + Copy , link : & str , ns : Option < Namespace > , is_inner_doc : bool ,) -> Option < DocLinkDef > { resolve_doc_path_on_ (db , link , def . attr_id () , ns , is_inner_doc) }
};
}
