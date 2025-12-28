macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnstableCTargetFeature {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unstable_ctarget_feature)] # [note] pub (crate) struct UnstableCTargetFeature < 'a > { pub feature : & 'a str , }
    };
}

UnstableCTargetFeature!()