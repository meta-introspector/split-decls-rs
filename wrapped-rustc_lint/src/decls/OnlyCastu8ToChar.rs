macro_rules! OnlyCastu8ToChar {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_only_cast_u8_to_char)] pub (crate) struct OnlyCastu8ToChar { # [suggestion (code = "'\\u{{{literal:X}}}'" , applicability = "machine-applicable")] pub span : Span , pub literal : u128 , }
    };
}

OnlyCastu8ToChar!()