macro_rules! InlineAsmUnsupportedTarget {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_inline_asm_unsupported_target , code = E0472)] pub (crate) struct InlineAsmUnsupportedTarget { # [primary_span] pub span : Span , }
    };
}

InlineAsmUnsupportedTarget!()