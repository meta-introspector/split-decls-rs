macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnknownCTargetFeaturePrefix {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unknown_ctarget_feature_prefix)] # [note] pub (crate) struct UnknownCTargetFeaturePrefix < 'a > { pub feature : & 'a str , }
    };
}

UnknownCTargetFeaturePrefix!();