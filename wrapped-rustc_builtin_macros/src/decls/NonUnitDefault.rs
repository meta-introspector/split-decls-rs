macro_rules! NonUnitDefault {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_non_unit_default)] # [help] pub (crate) struct NonUnitDefault { # [primary_span] pub (crate) span : Span , pub (crate) post : & 'static str , }
    };
}

NonUnitDefault!()