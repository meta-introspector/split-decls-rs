macro_rules! AsmAttributeNotSupported {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_attribute_not_supported)] pub (crate) struct AsmAttributeNotSupported { # [primary_span] pub (crate) span : Span , }
    };
}

AsmAttributeNotSupported!();