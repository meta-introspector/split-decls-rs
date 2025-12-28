macro_rules! UnusedBuiltinAttribute {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_builtin_attribute)] pub (crate) struct UnusedBuiltinAttribute { # [note] pub invoc_span : Span , pub attr_name : Symbol , pub macro_name : String , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub attr_span : Span , }
    };
}

UnusedBuiltinAttribute!();