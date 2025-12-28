macro_rules! deps {
    () => {
        SupertraitAsDerefTargetLabel!();
        LateContext!();
        SupertraitAsDerefTarget!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for DerefIntoDynSupertrait { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { let tcx = cx . tcx ; if let hir :: ItemKind :: Impl (impl_) = item . kind && let Some (of_trait) = & impl_ . of_trait && let Some (did) = of_trait . trait_ref . trait_def_id () && tcx . is_lang_item (did , LangItem :: Deref) && let self_ty = tcx . type_of (item . owner_id) . instantiate_identity () && let ty :: Dynamic (data , _ , ty :: Dyn) = self_ty . kind () && let Some (self_principal) = data . principal () && let Some (target) = cx . get_associated_type (self_ty , did , sym :: Target) && let ty :: Dynamic (data , _ , ty :: Dyn) = target . kind () && let Some (target_principal) = data . principal () && let Some (supertrait_principal) = supertraits (tcx , self_principal . with_self_ty (tcx , self_ty)) . find (| supertrait | supertrait . def_id () == target_principal . def_id ()) { let (self_ty , target_principal , supertrait_principal) = tcx . erase_and_anonymize_regions ((self_ty , target_principal , supertrait_principal)) ; let label2 = tcx . associated_items (item . owner_id) . find_by_ident_and_kind (tcx , Ident :: with_dummy_span (sym :: Target) , ty :: AssocTag :: Type , item . owner_id . to_def_id () ,) . map (| label | SupertraitAsDerefTargetLabel { label : tcx . def_span (label . def_id) }) ; let span = tcx . def_span (item . owner_id . def_id) ; cx . emit_span_lint (DEREF_INTO_DYN_SUPERTRAIT , span , SupertraitAsDerefTarget { self_ty , supertrait_principal : supertrait_principal . map_bound (| trait_ref | { ty :: ExistentialTraitRef :: erase_self_ty (tcx , trait_ref) }) , target_principal , label : span , label2 , } ,) ; } } }
    };
}

impl_159!()