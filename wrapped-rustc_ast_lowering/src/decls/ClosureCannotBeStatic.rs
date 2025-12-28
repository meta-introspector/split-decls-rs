macro_rules! ClosureCannotBeStatic {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_closure_cannot_be_static , code = E0697)] pub (crate) struct ClosureCannotBeStatic { # [primary_span] pub fn_decl_span : Span , }
    };
}

ClosureCannotBeStatic!()