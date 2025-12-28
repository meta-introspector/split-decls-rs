macro_rules! ConstHeapPtrInFinal {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_const_heap_ptr_in_final)] # [note] pub (crate) struct ConstHeapPtrInFinal { # [primary_span] pub span : Span , }
    };
}

ConstHeapPtrInFinal!();