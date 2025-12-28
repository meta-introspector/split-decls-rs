macro_rules! CoroutineTooManyParameters {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_coroutine_too_many_parameters , code = E0628)] pub (crate) struct CoroutineTooManyParameters { # [primary_span] pub fn_decl_span : Span , }
    };
}

CoroutineTooManyParameters!()