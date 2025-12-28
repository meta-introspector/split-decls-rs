macro_rules! deps {
    () => {
        DocsRangeMap!();
        Documentation!();
    };
}

macro_rules! HasDocs {
    () => {
        deps!();
        pub trait HasDocs : HasAttrs { fn docs (self , db : & dyn HirDatabase) -> Option < Documentation > ; fn docs_with_rangemap (self , db : & dyn HirDatabase) -> Option < (Documentation , DocsRangeMap) > ; fn resolve_doc_path (self , db : & dyn HirDatabase , link : & str , ns : Option < hir :: Namespace > , is_inner_doc : bool ,) -> Option < hir :: DocLinkDef > ; }
    };
}

HasDocs!()