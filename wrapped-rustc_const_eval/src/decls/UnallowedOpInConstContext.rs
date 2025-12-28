macro_rules! UnallowedOpInConstContext {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unallowed_op_in_const_context)] pub (crate) struct UnallowedOpInConstContext { # [primary_span] pub span : Span , pub msg : String , }
    };
}

UnallowedOpInConstContext!()