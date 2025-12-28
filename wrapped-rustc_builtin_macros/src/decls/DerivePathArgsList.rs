macro_rules! DerivePathArgsList {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_derive_path_args_list)] pub (crate) struct DerivePathArgsList { # [suggestion (code = "" , applicability = "machine-applicable")] # [primary_span] pub (crate) span : Span , }
    };
}

DerivePathArgsList!();