macro_rules! BadDeriveTarget {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_bad_derive_target , code = E0774)] pub (crate) struct BadDeriveTarget { # [primary_span] # [label] pub (crate) span : Span , # [label (builtin_macros_label2)] pub (crate) item : Span , }
    };
}

BadDeriveTarget!();