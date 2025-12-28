macro_rules! deps {
    () => {
        CollectedSizednessBounds!();
    };
}

macro_rules! collect_sizedness_bounds {
    () => {
        deps!();
        fn collect_sizedness_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , span : Span ,) -> CollectedSizednessBounds { let sized_did = tcx . require_lang_item (hir :: LangItem :: Sized , span) ; let sized = collect_bounds (hir_bounds , self_ty_where_predicates , sized_did) ; let meta_sized_did = tcx . require_lang_item (hir :: LangItem :: MetaSized , span) ; let meta_sized = collect_bounds (hir_bounds , self_ty_where_predicates , meta_sized_did) ; let pointee_sized_did = tcx . require_lang_item (hir :: LangItem :: PointeeSized , span) ; let pointee_sized = collect_bounds (hir_bounds , self_ty_where_predicates , pointee_sized_did) ; CollectedSizednessBounds { sized , meta_sized , pointee_sized } }
    };
}

collect_sizedness_bounds!();