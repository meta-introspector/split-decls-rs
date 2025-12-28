macro_rules! deps {
    () => {
        LateContext!();
        MissingDoc!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for MissingDoc { fn check_crate (& mut self , cx : & LateContext < '_ >) { self . check_missing_docs_attrs (cx , CRATE_DEF_ID , "the" , "crate") ; } fn check_item (& mut self , cx : & LateContext < '_ > , it : & hir :: Item < '_ >) { if let hir :: ItemKind :: Impl (..) | hir :: ItemKind :: Use (..) | hir :: ItemKind :: ExternCrate (..) = it . kind { return ; } let (article , desc) = cx . tcx . article_and_description (it . owner_id . to_def_id ()) ; self . check_missing_docs_attrs (cx , it . owner_id . def_id , article , desc) ; } fn check_trait_item (& mut self , cx : & LateContext < '_ > , trait_item : & hir :: TraitItem < '_ >) { let (article , desc) = cx . tcx . article_and_description (trait_item . owner_id . to_def_id ()) ; self . check_missing_docs_attrs (cx , trait_item . owner_id . def_id , article , desc) ; } fn check_impl_item (& mut self , cx : & LateContext < '_ > , impl_item : & hir :: ImplItem < '_ >) { let container = cx . tcx . associated_item (impl_item . owner_id . def_id) . container ; match container { AssocContainer :: TraitImpl (_) => return , AssocContainer :: Trait => { } AssocContainer :: InherentImpl => { let parent = cx . tcx . hir_get_parent_item (impl_item . hir_id ()) ; let impl_ty = cx . tcx . type_of (parent) . instantiate_identity () ; let outerdef = match impl_ty . kind () { ty :: Adt (def , _) => Some (def . did ()) , ty :: Foreign (def_id) => Some (* def_id) , _ => None , } ; let is_hidden = match outerdef { Some (id) => cx . tcx . is_doc_hidden (id) , None => false , } ; if is_hidden { return ; } } } let (article , desc) = cx . tcx . article_and_description (impl_item . owner_id . to_def_id ()) ; self . check_missing_docs_attrs (cx , impl_item . owner_id . def_id , article , desc) ; } fn check_foreign_item (& mut self , cx : & LateContext < '_ > , foreign_item : & hir :: ForeignItem < '_ >) { let (article , desc) = cx . tcx . article_and_description (foreign_item . owner_id . to_def_id ()) ; self . check_missing_docs_attrs (cx , foreign_item . owner_id . def_id , article , desc) ; } fn check_field_def (& mut self , cx : & LateContext < '_ > , sf : & hir :: FieldDef < '_ >) { if ! sf . is_positional () { self . check_missing_docs_attrs (cx , sf . def_id , "a" , "struct field") } } fn check_variant (& mut self , cx : & LateContext < '_ > , v : & hir :: Variant < '_ >) { self . check_missing_docs_attrs (cx , v . def_id , "a" , "variant") ; } }
    };
}

impl_32!();