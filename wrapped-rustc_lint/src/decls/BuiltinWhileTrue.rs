macro_rules! BuiltinWhileTrue {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_while_true)] pub (crate) struct BuiltinWhileTrue { # [suggestion (style = "short" , code = "{replace}" , applicability = "machine-applicable")] pub suggestion : Span , pub replace : String , }
    };
}

BuiltinWhileTrue!()