macro_rules! MacroExprFragment2024 {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_macro_expr_fragment_specifier_2024_migration)] pub (crate) struct MacroExprFragment2024 { # [suggestion (code = "expr_2021" , applicability = "machine-applicable")] pub suggestion : Span , }
    };
}

MacroExprFragment2024!()