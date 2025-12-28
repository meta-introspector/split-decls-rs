macro_rules! AsmUnsupportedOption {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_unsupported_option)] pub (crate) struct AsmUnsupportedOption { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , pub (crate) macro_name : & 'static str , }
    };
}

AsmUnsupportedOption!()