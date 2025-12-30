// Generated macro for CfgAccessibleInvalid (enum)
macro_rules! Depcrate_errorsCfgAccessibleInvalid {
() => {
// Module: crate::errors
// Provides: {"CfgAccessibleInvalid"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum CfgAccessibleInvalid { # [diag (builtin_macros_cfg_accessible_unspecified_path)] UnspecifiedPath (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_multiple_paths)] MultiplePaths (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_literal_path)] LiteralPath (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_has_args)] HasArguments (# [primary_span] Span) , }
};
}
