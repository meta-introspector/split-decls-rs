macro_rules! UnionPatMultipleFields {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_union_pat_multiple_fields)] pub (crate) struct UnionPatMultipleFields { # [primary_span] pub span : Span , }
    };
}

UnionPatMultipleFields!()