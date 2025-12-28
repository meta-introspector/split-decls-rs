macro_rules! AsmExpectedOther {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_expected_other)] pub (crate) struct AsmExpectedOther { # [primary_span] # [label (builtin_macros_expected_other)] pub (crate) span : Span , pub (crate) is_inline_asm : bool , }
    };
}

AsmExpectedOther!()