macro_rules! UnknownRustcCleanArgument {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_unknown_rustc_clean_argument)] pub (crate) struct UnknownRustcCleanArgument { # [primary_span] pub span : Span , }
    };
}

UnknownRustcCleanArgument!()