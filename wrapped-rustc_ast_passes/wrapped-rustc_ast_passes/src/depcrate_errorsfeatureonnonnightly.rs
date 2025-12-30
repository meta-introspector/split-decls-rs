// Generated macro for FeatureOnNonNightly (struct)
macro_rules! Depcrate_errorsFeatureOnNonNightly {
() => {
// Module: crate::errors
// Provides: {"FeatureOnNonNightly"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_feature_on_non_nightly , code = E0554)] pub (crate) struct FeatureOnNonNightly { # [primary_span] pub span : Span , pub channel : & 'static str , # [subdiagnostic] pub stable_features : Vec < StableFeature > , # [suggestion (code = "" , applicability = "machine-applicable")] pub sugg : Option < Span > , }
};
}
