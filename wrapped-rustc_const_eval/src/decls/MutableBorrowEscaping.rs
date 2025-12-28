macro_rules! MutableBorrowEscaping {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_mutable_borrow_escaping , code = E0764)] # [note] # [note (const_eval_note2)] # [help] pub (crate) struct MutableBorrowEscaping { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , }
    };
}

MutableBorrowEscaping!();