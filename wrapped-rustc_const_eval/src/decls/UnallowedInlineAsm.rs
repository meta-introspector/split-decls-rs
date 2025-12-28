macro_rules! UnallowedInlineAsm {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unallowed_inline_asm , code = E0015)] pub (crate) struct UnallowedInlineAsm { # [primary_span] pub span : Span , pub kind : ConstContext , }
    };
}

UnallowedInlineAsm!()