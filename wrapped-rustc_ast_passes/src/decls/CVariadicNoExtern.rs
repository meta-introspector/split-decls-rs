macro_rules! CVariadicNoExtern {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_c_variadic_no_extern)] # [help] pub (crate) struct CVariadicNoExtern { # [primary_span] pub span : Span , }
    };
}

CVariadicNoExtern!()