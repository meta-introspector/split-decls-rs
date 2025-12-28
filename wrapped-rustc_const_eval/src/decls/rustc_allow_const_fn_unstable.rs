macro_rules! rustc_allow_const_fn_unstable {
    () => {
        pub fn rustc_allow_const_fn_unstable (tcx : TyCtxt < '_ > , def_id : LocalDefId , feature_gate : Symbol ,) -> bool { let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind :: AllowConstFnUnstable (syms , _) if syms . contains (& feature_gate)) }
    };
}

rustc_allow_const_fn_unstable!()