macro_rules! deps {
    () => {
        DocsRangeMap!();
        HasDocs!();
        Documentation!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl HasDocs for hir :: ExternCrateDecl { fn docs (self , db : & dyn HirDatabase) -> Option < Documentation > { let crate_docs = docs_from_attrs (& self . resolved_crate (db) ? . root_module () . attrs (db)) ; let decl_docs = docs_from_attrs (& self . attrs (db)) ; match (decl_docs , crate_docs) { (None , None) => None , (Some (decl_docs) , None) => Some (decl_docs) , (None , Some (crate_docs)) => Some (crate_docs) , (Some (mut decl_docs) , Some (crate_docs)) => { decl_docs . push ('\n') ; decl_docs . push ('\n') ; decl_docs += & crate_docs ; Some (decl_docs) } } . map (Documentation :: new) } fn docs_with_rangemap (self , db : & dyn HirDatabase) -> Option < (Documentation , DocsRangeMap) > { let crate_docs = docs_with_rangemap (db , & self . resolved_crate (db) ? . root_module () . attrs (db)) ; let decl_docs = docs_with_rangemap (db , & self . attrs (db)) ; match (decl_docs , crate_docs) { (None , None) => None , (Some (decl_docs) , None) => Some (decl_docs) , (None , Some (crate_docs)) => Some (crate_docs) , (Some ((Documentation (mut decl_docs) , mut decl_range_map)) , Some ((Documentation (crate_docs) , crate_range_map)) ,) => { decl_docs . push ('\n') ; decl_docs . push ('\n') ; let offset = TextSize :: new (decl_docs . len () as u32) ; decl_docs += & crate_docs ; let crate_range_map = crate_range_map . shift_docstring_line_range (offset) ; decl_range_map . mapping . extend (crate_range_map . mapping) ; Some ((Documentation (decl_docs) , decl_range_map)) } } } fn resolve_doc_path (self , db : & dyn HirDatabase , link : & str , ns : Option < hir :: Namespace > , is_inner_doc : bool ,) -> Option < hir :: DocLinkDef > { resolve_doc_path_on (db , self , link , ns , is_inner_doc) } }
    };
}

impl_62!();