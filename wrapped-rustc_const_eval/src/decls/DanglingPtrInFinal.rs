macro_rules! deps {
    () => {
        InternKind!();
    };
}

macro_rules! DanglingPtrInFinal {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (const_eval_dangling_ptr_in_final)] pub (crate) struct DanglingPtrInFinal { # [primary_span] pub span : Span , pub kind : InternKind , }
    };
}

DanglingPtrInFinal!();