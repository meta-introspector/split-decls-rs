macro_rules! NonExhaustiveDefault {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_non_exhaustive_default)] # [help] pub (crate) struct NonExhaustiveDefault { # [primary_span] pub (crate) span : Span , # [label] pub (crate) non_exhaustive : Span , }
    };
}

NonExhaustiveDefault!()