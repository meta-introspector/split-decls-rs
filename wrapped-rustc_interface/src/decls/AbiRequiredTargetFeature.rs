macro_rules! AbiRequiredTargetFeature {
    () => {
        # [derive (Diagnostic)] # [diag (interface_abi_required_feature)] # [note] # [note (interface_abi_required_feature_issue)] pub (crate) struct AbiRequiredTargetFeature < 'a > { pub feature : & 'a str , pub enabled : & 'a str , }
    };
}

AbiRequiredTargetFeature!();