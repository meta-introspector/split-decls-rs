macro_rules! AllocErrorMustBeFn {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_alloc_error_must_be_fn)] pub (crate) struct AllocErrorMustBeFn { # [primary_span] pub (crate) span : Span , }
    };
}

AllocErrorMustBeFn!()