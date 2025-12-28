macro_rules! RegisterClassOnlyClobber {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_register_class_only_clobber)] pub (crate) struct RegisterClassOnlyClobber { # [primary_span] pub op_span : Span , pub reg_class_name : Symbol , }
    };
}

RegisterClassOnlyClobber!()