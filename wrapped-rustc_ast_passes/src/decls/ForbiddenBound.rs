macro_rules! ForbiddenBound {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_forbidden_bound)] pub (crate) struct ForbiddenBound { # [primary_span] pub spans : Vec < Span > , }
    };
}

ForbiddenBound!()