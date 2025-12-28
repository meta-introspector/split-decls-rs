macro_rules! RustCallIncorrectArgs {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_rustcall_incorrect_args)] pub (crate) struct RustCallIncorrectArgs { # [primary_span] pub span : Span , }
    };
}

RustCallIncorrectArgs!();