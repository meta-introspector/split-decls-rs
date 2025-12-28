macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MultipleExternalFuncDecl {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_multiple_external_func_decl)] pub (crate) struct MultipleExternalFuncDecl < 'a > { # [primary_span] pub span : Span , pub function : Symbol , pub library_name : & 'a str , }
    };
}

MultipleExternalFuncDecl!();