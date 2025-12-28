macro_rules! AsmModifierInvalid {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_modifier_invalid)] pub (crate) struct AsmModifierInvalid { # [primary_span] pub (crate) span : Span , }
    };
}

AsmModifierInvalid!();