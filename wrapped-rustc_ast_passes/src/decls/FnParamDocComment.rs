macro_rules! FnParamDocComment {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fn_param_doc_comment)] pub (crate) struct FnParamDocComment { # [primary_span] # [label] pub span : Span , }
    };
}

FnParamDocComment!()