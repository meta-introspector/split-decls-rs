macro_rules! CtorIsPrivate {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_ctor_is_private , code = E0603)] pub (crate) struct CtorIsPrivate { # [primary_span] pub span : Span , pub def : String , }
    };
}

CtorIsPrivate!();