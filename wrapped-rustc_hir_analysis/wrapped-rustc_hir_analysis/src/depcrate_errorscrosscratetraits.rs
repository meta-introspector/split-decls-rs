// Generated macro for CrossCrateTraits (struct)
macro_rules! Depcrate_errorsCrossCrateTraits {
() => {
// Module: crate::errors
// Provides: {"CrossCrateTraits"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_cross_crate_traits , code = E0321)] pub (crate) struct CrossCrateTraits < 'a > { # [primary_span] # [label] pub span : Span , pub traits : String , pub self_ty : Ty < 'a > , }
};
}
