macro_rules! deps {
    () => {
        ForbiddenTargetFeatureAttr!();
        Aarch64SoftfloatNeon!();
        FeatureNotValid!();
    };
}

macro_rules! from_target_feature_attr {
    () => {
        deps!();
        # [doc = " Compute the enabled target features from the `#[target_feature]` function attribute."] # [doc = " Enabled target features are added to `target_features`."] pub (crate) fn from_target_feature_attr (tcx : TyCtxt < '_ > , did : LocalDefId , features : & [(Symbol , Span)] , was_forced : bool , rust_target_features : & UnordMap < String , target_features :: Stability > , target_features : & mut Vec < TargetFeature > ,) { let rust_features = tcx . features () ; let abi_feature_constraints = tcx . sess . target . abi_required_features () ; for & (feature , feature_span) in features { let feature_str = feature . as_str () ; let Some (stability) = rust_target_features . get (feature_str) else { let plus_hint = feature_str . strip_prefix ('+') . is_some_and (| stripped | rust_target_features . contains_key (stripped)) ; tcx . dcx () . emit_err (FeatureNotValid { feature : feature_str , span : feature_span , plus_hint , }) ; continue ; } ; if let Err (reason) = stability . toggle_allowed () { tcx . dcx () . emit_err (errors :: ForbiddenTargetFeatureAttr { span : feature_span , feature : feature_str , reason , }) ; } else if let Some (nightly_feature) = stability . requires_nightly () && ! rust_features . enabled (nightly_feature) { feature_err (& tcx . sess , nightly_feature , feature_span , format ! ("the target feature `{feature}` is currently unstable") ,) . emit () ; } else { for & name in tcx . implied_target_features (feature) { if ! tcx . sess . opts . actually_rustdoc { if abi_feature_constraints . incompatible . contains (& name . as_str ()) { if tcx . sess . target . arch == "aarch64" && name . as_str () == "neon" { tcx . emit_node_span_lint (AARCH64_SOFTFLOAT_NEON , tcx . local_def_id_to_hir_id (did) , feature_span , errors :: Aarch64SoftfloatNeon ,) ; } else { tcx . dcx () . emit_err (errors :: ForbiddenTargetFeatureAttr { span : feature_span , feature : name . as_str () , reason : "this feature is incompatible with the target ABI" , }) ; } } } let kind = if name != feature { TargetFeatureKind :: Implied } else if was_forced { TargetFeatureKind :: Forced } else { TargetFeatureKind :: Enabled } ; target_features . push (TargetFeature { name , kind }) } } } }
    };
}

from_target_feature_attr!();