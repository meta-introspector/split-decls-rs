// Generated macro for impl_91 (impl)
macro_rules! Depcrate_documentationimpl_91 {
() => {
// Module: crate::documentation
// Provides: {"impl_91"}
// Dependencies: {}
impl HasDocs for hir :: AssocItem { fn docs (self , db : & dyn HirDatabase) -> Option < Documentation > { match self { hir :: AssocItem :: Function (it) => it . docs (db) , hir :: AssocItem :: Const (it) => it . docs (db) , hir :: AssocItem :: TypeAlias (it) => it . docs (db) , } } fn docs_with_rangemap (self , db : & dyn HirDatabase) -> Option < (Documentation , DocsRangeMap) > { match self { hir :: AssocItem :: Function (it) => it . docs_with_rangemap (db) , hir :: AssocItem :: Const (it) => it . docs_with_rangemap (db) , hir :: AssocItem :: TypeAlias (it) => it . docs_with_rangemap (db) , } } fn resolve_doc_path (self , db : & dyn HirDatabase , link : & str , ns : Option < hir :: Namespace > , is_inner_doc : bool ,) -> Option < hir :: DocLinkDef > { match self { hir :: AssocItem :: Function (it) => it . resolve_doc_path (db , link , ns , is_inner_doc) , hir :: AssocItem :: Const (it) => it . resolve_doc_path (db , link , ns , is_inner_doc) , hir :: AssocItem :: TypeAlias (it) => it . resolve_doc_path (db , link , ns , is_inner_doc) , } } }
};
}
