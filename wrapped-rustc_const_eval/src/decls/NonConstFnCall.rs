macro_rules! NonConstFnCall {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_fn_call , code = E0015)] pub (crate) struct NonConstFnCall { # [primary_span] pub span : Span , pub def_path_str : String , pub def_descr : & 'static str , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstFnCall!();