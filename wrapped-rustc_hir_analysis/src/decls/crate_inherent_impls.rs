macro_rules! deps {
    () => {
        InherentCollect!();
    };
}

macro_rules! crate_inherent_impls {
    () => {
        deps!();
        # [doc = " On-demand query: yields a map containing all types mapped to their inherent impls."] pub (crate) fn crate_inherent_impls (tcx : TyCtxt < '_ > , () : () ,) -> (& '_ CrateInherentImpls , Result < () , ErrorGuaranteed >) { let mut collect = InherentCollect { tcx , impls_map : Default :: default () } ; let mut res = Ok (()) ; for id in tcx . hir_free_items () { res = res . and (collect . check_item (id)) ; } (tcx . arena . alloc (collect . impls_map) , res) }
    };
}

crate_inherent_impls!();