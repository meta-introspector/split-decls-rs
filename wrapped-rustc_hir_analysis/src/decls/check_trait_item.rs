macro_rules! check_trait_item {
    () => {
        pub (crate) fn check_trait_item < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { lint_item_shadowing_supertrait_item (tcx , def_id) ; let mut res = Ok (()) ; if matches ! (tcx . def_kind (def_id) , DefKind :: AssocFn) { for & assoc_ty_def_id in tcx . associated_types_for_impl_traits_in_associated_fn (def_id . to_def_id ()) { res = res . and (check_associated_item (tcx , assoc_ty_def_id . expect_local ())) ; } } res }
    };
}

check_trait_item!()