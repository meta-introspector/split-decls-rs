macro_rules! deps {
    () => {
        ItemCtxt!();
    };
}

macro_rules! impl_trait_header {
    () => {
        deps!();
        fn impl_trait_header (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ty :: ImplTraitHeader < '_ > > { let icx = ItemCtxt :: new (tcx , def_id) ; let item = tcx . hir_expect_item (def_id) ; let impl_ = item . expect_impl () ; let is_rustc_reservation = tcx . has_attr (def_id , sym :: rustc_reservation_impl) ; if is_rustc_reservation && impl_ . of_trait . is_none () { tcx . dcx () . span_err (item . span , "reservation impls can't be inherent") ; } impl_ . of_trait . map (| of_trait | { let selfty = tcx . type_of (def_id) . instantiate_identity () ; check_impl_constness (tcx , of_trait . constness , & of_trait . trait_ref) ; let trait_ref = icx . lowerer () . lower_impl_trait_ref (& of_trait . trait_ref , selfty) ; ty :: ImplTraitHeader { trait_ref : ty :: EarlyBinder :: bind (trait_ref) , safety : of_trait . safety , polarity : polarity_of_impl (tcx , of_trait , is_rustc_reservation) , constness : of_trait . constness , } }) }
    };
}

impl_trait_header!()