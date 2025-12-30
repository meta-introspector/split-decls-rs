// Generated macro for impl_has_docs_enum (macro)
macro_rules! Depcrate_documentationimpl_has_docs_enum {
() => {
// Module: crate::documentation
// Provides: {"impl_has_docs_enum"}
// Dependencies: {}
macro_rules ! impl_has_docs_enum { ($ ($ variant : ident) ,* for $ enum : ident) => { $ (impl HasDocs for hir ::$ variant { fn docs (self , db : & dyn HirDatabase) -> Option < Documentation > { hir ::$ enum ::$ variant (self) . docs (db) } fn docs_with_rangemap (self , db : & dyn HirDatabase ,) -> Option < (Documentation , DocsRangeMap) > { hir ::$ enum ::$ variant (self) . docs_with_rangemap (db) } fn resolve_doc_path (self , db : & dyn HirDatabase , link : & str , ns : Option < hir :: Namespace >, is_inner_doc : bool ,) -> Option < hir :: DocLinkDef > { hir ::$ enum ::$ variant (self) . resolve_doc_path (db , link , ns , is_inner_doc) } }) * } ; }
};
}
