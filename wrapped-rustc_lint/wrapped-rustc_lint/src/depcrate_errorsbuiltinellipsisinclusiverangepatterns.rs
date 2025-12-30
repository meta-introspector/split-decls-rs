// Generated macro for BuiltinEllipsisInclusiveRangePatterns (struct)
macro_rules! Depcrate_errorsBuiltinEllipsisInclusiveRangePatterns {
() => {
// Module: crate::errors
// Provides: {"BuiltinEllipsisInclusiveRangePatterns"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (lint_builtin_ellipsis_inclusive_range_patterns , code = E0783)] pub (crate) struct BuiltinEllipsisInclusiveRangePatterns { # [primary_span] pub span : Span , # [suggestion (style = "short" , code = "{replace}" , applicability = "machine-applicable")] pub suggestion : Span , pub replace : String , }
};
}
