// Generated macro for InvalidLegacyConstGenericArg (struct)
macro_rules! Depcrate_errorsInvalidLegacyConstGenericArg {
() => {
// Module: crate::errors
// Provides: {"InvalidLegacyConstGenericArg"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_invalid_legacy_const_generic_arg)] pub (crate) struct InvalidLegacyConstGenericArg { # [primary_span] pub span : Span , # [subdiagnostic] pub suggestion : UseConstGenericArg , }
};
}
