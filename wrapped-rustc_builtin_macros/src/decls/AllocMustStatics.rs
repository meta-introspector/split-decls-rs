macro_rules! AllocMustStatics {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_alloc_must_statics)] pub (crate) struct AllocMustStatics { # [primary_span] pub (crate) span : Span , }
    };
}

AllocMustStatics!();