macro_rules! deps {
    () => {
        PossibleFeature!();
        Diagnostic!();
    };
}

macro_rules! UnknownCTargetFeature {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unknown_ctarget_feature)] # [note] pub (crate) struct UnknownCTargetFeature < 'a > { pub feature : & 'a str , # [subdiagnostic] pub rust_feature : PossibleFeature < 'a > , }
    };
}

UnknownCTargetFeature!()