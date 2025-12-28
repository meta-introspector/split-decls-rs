macro_rules! TupleStructWithDefault {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_default_field_in_tuple)] pub (crate) struct TupleStructWithDefault { # [primary_span] # [label] pub span : Span , }
    };
}

TupleStructWithDefault!()