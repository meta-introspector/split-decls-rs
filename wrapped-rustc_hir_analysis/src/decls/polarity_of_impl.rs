macro_rules! polarity_of_impl {
    () => {
        fn polarity_of_impl (tcx : TyCtxt < '_ > , of_trait : & hir :: TraitImplHeader < '_ > , is_rustc_reservation : bool ,) -> ty :: ImplPolarity { match of_trait . polarity { hir :: ImplPolarity :: Negative (span) => { if is_rustc_reservation { let span = span . to (of_trait . trait_ref . path . span) ; tcx . dcx () . span_err (span , "reservation impls can't be negative") ; } ty :: ImplPolarity :: Negative } hir :: ImplPolarity :: Positive => { if is_rustc_reservation { ty :: ImplPolarity :: Reservation } else { ty :: ImplPolarity :: Positive } } } }
    };
}

polarity_of_impl!()