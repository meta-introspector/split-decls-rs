macro_rules! AsmExplicitRegisterName {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_explicit_register_name)] pub (crate) struct AsmExplicitRegisterName { # [primary_span] pub (crate) span : Span , }
    };
}

AsmExplicitRegisterName!()