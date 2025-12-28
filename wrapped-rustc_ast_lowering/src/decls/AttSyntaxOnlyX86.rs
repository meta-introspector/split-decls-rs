macro_rules! AttSyntaxOnlyX86 {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_att_syntax_only_x86)] pub (crate) struct AttSyntaxOnlyX86 { # [primary_span] pub span : Span , }
    };
}

AttSyntaxOnlyX86!()