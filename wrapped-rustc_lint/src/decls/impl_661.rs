macro_rules! deps {
    () => {
        MultipleSupertraitUpcastable!();
        LateContext!();
    };
}

macro_rules! impl_661 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for MultipleSupertraitUpcastable { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { let def_id = item . owner_id . to_def_id () ; if let hir :: ItemKind :: Trait (_ , _ , _ , ident , ..) = item . kind && cx . tcx . is_dyn_compatible (def_id) { let direct_super_traits_iter = cx . tcx . explicit_super_predicates_of (def_id) . iter_identity_copied () . filter_map (| (pred , _) | pred . as_trait_clause ()) . filter (| pred | ! cx . tcx . is_lang_item (pred . def_id () , hir :: LangItem :: MetaSized)) . filter (| pred | ! cx . tcx . is_default_trait (pred . def_id ())) ; if direct_super_traits_iter . count () > 1 { cx . emit_span_lint (MULTIPLE_SUPERTRAIT_UPCASTABLE , cx . tcx . def_span (def_id) , crate :: lints :: MultipleSupertraitUpcastable { ident } ,) ; } } } }
    };
}

impl_661!()