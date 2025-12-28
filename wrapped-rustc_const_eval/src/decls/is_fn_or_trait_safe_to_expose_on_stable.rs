macro_rules! is_fn_or_trait_safe_to_expose_on_stable {
    () => {
        # [doc = " Returns `true` if the given `def_id` (trait or function) is \"safe to expose on stable\"."] # [doc = ""] # [doc = " This is relevant within a `staged_api` crate. Unlike with normal features, the use of unstable"] # [doc = " const features *recursively* taints the functions that use them. This is to avoid accidentally"] # [doc = " exposing e.g. the implementation of an unstable const intrinsic on stable. So we partition the"] # [doc = " world into two functions: those that are safe to expose on stable (and hence may not use"] # [doc = " unstable features, not even recursively), and those that are not."] pub fn is_fn_or_trait_safe_to_expose_on_stable (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { if tcx . is_const_default_method (def_id) { return is_fn_or_trait_safe_to_expose_on_stable (tcx , tcx . parent (def_id)) ; } match tcx . lookup_const_stability (def_id) { None => { def_id . is_local () && tcx . features () . staged_api () } Some (stab) => { stab . is_const_stable () || stab . const_stable_indirect } } }
    };
}

is_fn_or_trait_safe_to_expose_on_stable!();