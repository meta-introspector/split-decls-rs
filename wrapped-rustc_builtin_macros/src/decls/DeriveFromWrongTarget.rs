macro_rules! DeriveFromWrongTarget {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_derive_from_wrong_target)] # [note (builtin_macros_derive_from_usage_note)] pub (crate) struct DeriveFromWrongTarget < 'a > { # [primary_span] pub (crate) span : MultiSpan , pub (crate) kind : & 'a str , }
    };
}

DeriveFromWrongTarget!();