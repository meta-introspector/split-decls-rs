macro_rules! NakedFunctionsMustNakedAsm {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_naked_functions_must_naked_asm , code = E0787)] pub (crate) struct NakedFunctionsMustNakedAsm { # [primary_span] # [label] pub span : Span , }
    };
}

NakedFunctionsMustNakedAsm!();