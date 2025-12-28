macro_rules! deps {
    () => {
        InherentOverlapChecker!();
    };
}

macro_rules! crate_inherent_impls_overlap_check {
    () => {
        deps!();
        pub (crate) fn crate_inherent_impls_overlap_check (tcx : TyCtxt < '_ > , () : () ,) -> Result < () , ErrorGuaranteed > { let mut inherent_overlap_checker = InherentOverlapChecker { tcx } ; let mut res = Ok (()) ; for id in tcx . hir_free_items () { res = res . and (inherent_overlap_checker . check_item (id)) ; } res }
    };
}

crate_inherent_impls_overlap_check!()