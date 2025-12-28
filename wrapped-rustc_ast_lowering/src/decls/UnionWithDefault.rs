macro_rules! UnionWithDefault {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_union_default_field_values)] pub (crate) struct UnionWithDefault { # [primary_span] pub span : Span , }
    };
}

UnionWithDefault!()