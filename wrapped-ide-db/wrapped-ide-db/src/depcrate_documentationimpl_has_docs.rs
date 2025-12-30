// Generated macro for impl_has_docs (macro)
macro_rules! Depcrate_documentationimpl_has_docs {
() => {
// Module: crate::documentation
// Provides: {"impl_has_docs"}
// Dependencies: {}
macro_rules ! impl_has_docs { ($ ($ def : ident ,) *) => { $ (impl HasDocs for hir ::$ def { fn docs (self , db : & dyn HirDatabase) -> Option < Documentation > { docs_from_attrs (& self . attrs (db)) . map (Documentation) } fn docs_with_rangemap (self , db : & dyn HirDatabase ,) -> Option < (Documentation , DocsRangeMap) > { docs_with_rangemap (db , & self . attrs (db)) } fn resolve_doc_path (self , db : & dyn HirDatabase , link : & str , ns : Option < hir :: Namespace >, is_inner_doc : bool ,) -> Option < hir :: DocLinkDef > { resolve_doc_path_on (db , self , link , ns , is_inner_doc) } }) * } ; }
};
}
