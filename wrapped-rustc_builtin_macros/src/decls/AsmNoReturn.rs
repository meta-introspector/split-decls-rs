macro_rules! AsmNoReturn {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_noreturn)] pub (crate) struct AsmNoReturn { # [primary_span] pub (crate) outputs_sp : Vec < Span > , }
    };
}

AsmNoReturn!();