macro_rules! ExportMacroRules {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_export_macro_rules)] pub (crate) struct ExportMacroRules { # [primary_span] pub (crate) span : Span , }
    };
}

ExportMacroRules!()