macro_rules! NonConstFmtMacroCall {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_fmt_macro_call , code = E0015)] pub (crate) struct NonConstFmtMacroCall { # [primary_span] pub span : Span , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstFmtMacroCall!();