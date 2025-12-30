// Generated macro for ForbiddenCTargetFeature (struct)
macro_rules! Depcrate_errorsForbiddenCTargetFeature {
() => {
// Module: crate::errors
// Provides: {"ForbiddenCTargetFeature"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_forbidden_ctarget_feature)] # [note] # [note (codegen_ssa_forbidden_ctarget_feature_issue)] pub (crate) struct ForbiddenCTargetFeature < 'a > { pub feature : & 'a str , pub enabled : & 'a str , pub reason : & 'a str , }
};
}
