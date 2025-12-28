macro_rules! ArgMismatchIndeterminate {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_arg_mismatch_indeterminate)] pub (crate) struct ArgMismatchIndeterminate { # [primary_span] pub span : Span , }
    };
}

ArgMismatchIndeterminate!()