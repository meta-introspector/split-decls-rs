macro_rules! AsmPureNoOutput {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_pure_no_output)] pub (crate) struct AsmPureNoOutput { # [primary_span] pub (crate) spans : Vec < Span > , }
    };
}

AsmPureNoOutput!()