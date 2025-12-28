macro_rules! CVariadicBadExtern {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_c_variadic_bad_extern)] # [help] pub (crate) struct CVariadicBadExtern { # [primary_span] pub span : Span , pub abi : Symbol , # [label] pub extern_span : Span , }
    };
}

CVariadicBadExtern!()