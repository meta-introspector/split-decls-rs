macro_rules! crate_inherent_impls_validity_check {
    () => {
        pub (crate) fn crate_inherent_impls_validity_check (tcx : TyCtxt < '_ > , () : () ,) -> Result < () , ErrorGuaranteed > { tcx . crate_inherent_impls (()) . 1 }
    };
}

crate_inherent_impls_validity_check!();