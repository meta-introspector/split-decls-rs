macro_rules! RawPtrComparisonErr {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_raw_ptr_comparison)] # [note] pub (crate) struct RawPtrComparisonErr { # [primary_span] pub span : Span , }
    };
}

RawPtrComparisonErr!();