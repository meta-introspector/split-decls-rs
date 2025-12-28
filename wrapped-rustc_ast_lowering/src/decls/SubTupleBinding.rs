macro_rules! SubTupleBinding {
    () => {
        # [derive (Diagnostic)] # [help] # [diag (ast_lowering_sub_tuple_binding)] pub (crate) struct SubTupleBinding < 'a > { # [primary_span] # [label] # [suggestion (ast_lowering_sub_tuple_binding_suggestion , style = "verbose" , code = ".." , applicability = "maybe-incorrect")] pub span : Span , pub ident : Ident , pub ident_name : Symbol , pub ctx : & 'a str , }
    };
}

SubTupleBinding!()