macro_rules! AsmMayUnwind {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_mayunwind)] pub (crate) struct AsmMayUnwind { # [primary_span] pub (crate) labels_sp : Vec < Span > , }
    };
}

AsmMayUnwind!()