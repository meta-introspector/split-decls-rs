macro_rules! is_promotable_const_fn {
    () => {
        fn is_promotable_const_fn (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . is_const_fn (def_id) && match tcx . lookup_const_stability (def_id) { Some (stab) => { if cfg ! (debug_assertions) && stab . promotable { let sig = tcx . fn_sig (def_id) ; assert ! (sig . skip_binder () . safety () . is_safe () , "don't mark const unsafe fns as promotable" ,) ; } stab . promotable } None => false , } }
    };
}

is_promotable_const_fn!()