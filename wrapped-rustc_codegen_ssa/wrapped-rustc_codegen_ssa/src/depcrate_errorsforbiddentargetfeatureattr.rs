// Generated macro for ForbiddenTargetFeatureAttr (struct)
macro_rules! Depcrate_errorsForbiddenTargetFeatureAttr {
() => {
// Module: crate::errors
// Provides: {"ForbiddenTargetFeatureAttr"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_forbidden_target_feature_attr)] pub struct ForbiddenTargetFeatureAttr < 'a > { # [primary_span] pub span : Span , pub feature : & 'a str , pub reason : & 'a str , }
};
}
