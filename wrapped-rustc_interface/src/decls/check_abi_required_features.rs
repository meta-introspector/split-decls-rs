macro_rules! deps {
    () => {
        AbiRequiredTargetFeature!();
    };
}

macro_rules! check_abi_required_features {
    () => {
        deps!();
        # [doc = " Ensures that all target features required by the ABI are present."] # [doc = " Must be called after `unstable_target_features` has been populated!"] pub (crate) fn check_abi_required_features (sess : & Session) { let abi_feature_constraints = sess . target . abi_required_features () ; for feature in abi_feature_constraints . required . iter () . chain (abi_feature_constraints . incompatible . iter ()) { assert ! (sess . target . rust_target_features () . iter () . any (| (name , ..) | feature == name) , "target feature {feature} is required/incompatible for the current ABI but not a recognized feature for this target") ; } for feature in abi_feature_constraints . required { if ! sess . unstable_target_features . contains (& Symbol :: intern (feature)) { sess . dcx () . emit_warn (errors :: AbiRequiredTargetFeature { feature , enabled : "enabled" }) ; } } for feature in abi_feature_constraints . incompatible { if sess . unstable_target_features . contains (& Symbol :: intern (feature)) { sess . dcx () . emit_warn (errors :: AbiRequiredTargetFeature { feature , enabled : "disabled" }) ; } } }
    };
}

check_abi_required_features!();