macro_rules! NonGenericPointee {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_non_generic_pointee)] pub (crate) struct NonGenericPointee { # [primary_span] pub span : Span , }
    };
}

NonGenericPointee!();