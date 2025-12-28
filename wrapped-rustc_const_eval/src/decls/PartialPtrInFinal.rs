macro_rules! deps {
    () => {
        InternKind!();
    };
}

macro_rules! PartialPtrInFinal {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (const_eval_partial_pointer_in_final)] # [note] pub (crate) struct PartialPtrInFinal { # [primary_span] pub span : Span , pub kind : InternKind , }
    };
}

PartialPtrInFinal!()