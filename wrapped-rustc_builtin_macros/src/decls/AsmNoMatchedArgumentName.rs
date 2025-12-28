macro_rules! AsmNoMatchedArgumentName {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_no_matched_argument_name)] pub (crate) struct AsmNoMatchedArgumentName { pub (crate) name : String , # [primary_span] pub (crate) span : Span , }
    };
}

AsmNoMatchedArgumentName!()