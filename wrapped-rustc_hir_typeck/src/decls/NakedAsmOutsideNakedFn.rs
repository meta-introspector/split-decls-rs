macro_rules! NakedAsmOutsideNakedFn {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_naked_asm_outside_naked_fn)] pub (crate) struct NakedAsmOutsideNakedFn { # [primary_span] pub span : Span , }
    };
}

NakedAsmOutsideNakedFn!();