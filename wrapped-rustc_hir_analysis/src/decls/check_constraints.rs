macro_rules! deps {
    () => {
        UnsupportedDelegation!();
    };
}

macro_rules! check_constraints {
    () => {
        deps!();
        fn check_constraints < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , sig_id : DefId ,) -> Result < () , ErrorGuaranteed > { let mut ret = Ok (()) ; let mut emit = | descr | { ret = Err (tcx . dcx () . emit_err (crate :: errors :: UnsupportedDelegation { span : tcx . def_span (def_id) , descr , callee_span : tcx . def_span (sig_id) , })) ; } ; if let Some (local_sig_id) = sig_id . as_local () && tcx . hir_opt_delegation_sig_id (local_sig_id) . is_some () { emit ("recursive delegation is not supported yet") ; } if tcx . fn_sig (sig_id) . skip_binder () . skip_binder () . c_variadic { emit ("delegation to C-variadic functions is not allowed") ; } ret }
    };
}

check_constraints!()