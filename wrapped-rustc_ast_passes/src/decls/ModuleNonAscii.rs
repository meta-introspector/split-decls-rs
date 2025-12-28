macro_rules! ModuleNonAscii {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_module_nonascii , code = E0754)] # [help] pub (crate) struct ModuleNonAscii { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

ModuleNonAscii!()