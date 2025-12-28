macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ForbiddenTargetFeatureAttr {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_forbidden_target_feature_attr)] pub struct ForbiddenTargetFeatureAttr < 'a > { # [primary_span] pub span : Span , pub feature : & 'a str , pub reason : & 'a str , }
    };
}

ForbiddenTargetFeatureAttr!()