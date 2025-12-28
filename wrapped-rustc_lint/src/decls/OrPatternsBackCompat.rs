macro_rules! OrPatternsBackCompat {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_or_patterns_back_compat)] pub (crate) struct OrPatternsBackCompat { # [suggestion (code = "{suggestion}" , applicability = "machine-applicable")] pub span : Span , pub suggestion : String , }
    };
}

OrPatternsBackCompat!()