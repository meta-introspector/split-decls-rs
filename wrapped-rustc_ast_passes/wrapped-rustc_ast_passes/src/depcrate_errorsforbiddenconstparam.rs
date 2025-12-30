// Generated macro for ForbiddenConstParam (struct)
macro_rules! Depcrate_errorsForbiddenConstParam {
() => {
// Module: crate::errors
// Provides: {"ForbiddenConstParam"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_forbidden_const_param)] pub (crate) struct ForbiddenConstParam { # [primary_span] pub const_param_spans : Vec < Span > , }
};
}
