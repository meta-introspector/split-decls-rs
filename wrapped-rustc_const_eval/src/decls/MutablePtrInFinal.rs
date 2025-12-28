macro_rules! deps {
    () => {
        InternKind!();
    };
}

macro_rules! MutablePtrInFinal {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (const_eval_mutable_ptr_in_final)] pub (crate) struct MutablePtrInFinal { # [primary_span] pub span : Span , pub kind : InternKind , }
    };
}

MutablePtrInFinal!()