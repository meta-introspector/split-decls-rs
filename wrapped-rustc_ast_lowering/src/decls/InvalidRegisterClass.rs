macro_rules! InvalidRegisterClass {
    () => {
        # [derive (Diagnostic)] # [note] # [diag (ast_lowering_invalid_register_class)] pub (crate) struct InvalidRegisterClass { # [primary_span] pub op_span : Span , pub reg_class : Symbol , pub supported_register_classes : String , }
    };
}

InvalidRegisterClass!();