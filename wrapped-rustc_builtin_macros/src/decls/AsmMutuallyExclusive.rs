macro_rules! AsmMutuallyExclusive {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_mutually_exclusive)] pub (crate) struct AsmMutuallyExclusive { # [primary_span] pub (crate) spans : Vec < Span > , pub (crate) opt1 : & 'static str , pub (crate) opt2 : & 'static str , }
    };
}

AsmMutuallyExclusive!()