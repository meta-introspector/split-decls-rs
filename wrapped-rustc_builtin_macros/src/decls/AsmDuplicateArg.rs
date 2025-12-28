macro_rules! AsmDuplicateArg {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_duplicate_arg)] pub (crate) struct AsmDuplicateArg { # [primary_span] # [label (builtin_macros_arg)] pub (crate) span : Span , # [label] pub (crate) prev : Span , pub (crate) name : Symbol , }
    };
}

AsmDuplicateArg!();