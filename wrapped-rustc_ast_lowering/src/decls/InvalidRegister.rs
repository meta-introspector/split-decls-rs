macro_rules! InvalidRegister {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_invalid_register)] pub (crate) struct InvalidRegister < 'a > { # [primary_span] pub op_span : Span , pub reg : Symbol , pub error : & 'a str , }
    };
}

InvalidRegister!()