macro_rules! AsmPureCombine {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_pure_combine)] pub (crate) struct AsmPureCombine { # [primary_span] pub (crate) spans : Vec < Span > , }
    };
}

AsmPureCombine!();