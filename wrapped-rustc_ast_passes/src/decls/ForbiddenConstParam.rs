macro_rules! ForbiddenConstParam {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_forbidden_const_param)] pub (crate) struct ForbiddenConstParam { # [primary_span] pub const_param_spans : Vec < Span > , }
    };
}

ForbiddenConstParam!()