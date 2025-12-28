macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! TargetFeatureSafeTrait {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_target_feature_safe_trait)] pub (crate) struct TargetFeatureSafeTrait { # [primary_span] # [label] pub span : Span , # [label (codegen_ssa_label_def)] pub def : Span , }
    };
}

TargetFeatureSafeTrait!()