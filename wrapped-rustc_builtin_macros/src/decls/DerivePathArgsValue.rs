macro_rules! DerivePathArgsValue {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_derive_path_args_value)] pub (crate) struct DerivePathArgsValue { # [suggestion (code = "" , applicability = "machine-applicable")] # [primary_span] pub (crate) span : Span , }
    };
}

DerivePathArgsValue!()