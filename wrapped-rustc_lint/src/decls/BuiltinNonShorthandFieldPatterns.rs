macro_rules! BuiltinNonShorthandFieldPatterns {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_non_shorthand_field_patterns)] pub (crate) struct BuiltinNonShorthandFieldPatterns { pub ident : Ident , # [suggestion (code = "{prefix}{ident}" , applicability = "machine-applicable")] pub suggestion : Span , pub prefix : & 'static str , }
    };
}

BuiltinNonShorthandFieldPatterns!();