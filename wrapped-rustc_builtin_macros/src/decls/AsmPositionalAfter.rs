macro_rules! AsmPositionalAfter {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_pos_after)] pub (crate) struct AsmPositionalAfter { # [primary_span] # [label (builtin_macros_pos)] pub (crate) span : Span , # [label (builtin_macros_named)] pub (crate) named : Vec < Span > , # [label (builtin_macros_explicit)] pub (crate) explicit : Vec < Span > , }
    };
}

AsmPositionalAfter!()