macro_rules! RawPtrToIntErr {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_raw_ptr_to_int)] # [note] # [note (const_eval_note2)] pub (crate) struct RawPtrToIntErr { # [primary_span] pub span : Span , }
    };
}

RawPtrToIntErr!()