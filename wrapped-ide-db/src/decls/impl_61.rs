macro_rules! deps {
    () => {
        HasDocs!();
        DocsRangeMap!();
        Documentation!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl HasDocs for hir :: AssocItem { fn docs (self , db : & dyn HirDatabase) -> Option < Documentation > { match self { hir :: AssocItem :: Function (it) => it . docs (db) , hir :: AssocItem :: Const (it) => it . docs (db) , hir :: AssocItem :: TypeAlias (it) => it . docs (db) , } } fn docs_with_rangemap (self , db : & dyn HirDatabase) -> Option < (Documentation , DocsRangeMap) > { match self { hir :: AssocItem :: Function (it) => it . docs_with_rangemap (db) , hir :: AssocItem :: Const (it) => it . docs_with_rangemap (db) , hir :: AssocItem :: TypeAlias (it) => it . docs_with_rangemap (db) , } } fn resolve_doc_path (self , db : & dyn HirDatabase , link : & str , ns : Option < hir :: Namespace > , is_inner_doc : bool ,) -> Option < hir :: DocLinkDef > { match self { hir :: AssocItem :: Function (it) => it . resolve_doc_path (db , link , ns , is_inner_doc) , hir :: AssocItem :: Const (it) => it . resolve_doc_path (db , link , ns , is_inner_doc) , hir :: AssocItem :: TypeAlias (it) => it . resolve_doc_path (db , link , ns , is_inner_doc) , } } }
    };
}

impl_61!();