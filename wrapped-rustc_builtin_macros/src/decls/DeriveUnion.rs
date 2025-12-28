macro_rules! DeriveUnion {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_cannot_derive_union)] pub (crate) struct DeriveUnion { # [primary_span] pub (crate) span : Span , }
    };
}

DeriveUnion!()