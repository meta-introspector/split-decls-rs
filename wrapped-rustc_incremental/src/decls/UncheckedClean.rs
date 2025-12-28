macro_rules! UncheckedClean {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_unchecked_clean)] pub (crate) struct UncheckedClean { # [primary_span] pub span : Span , }
    };
}

UncheckedClean!()