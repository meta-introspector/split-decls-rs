macro_rules! BuiltinNoMangleGeneric {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_no_mangle_generic)] pub (crate) struct BuiltinNoMangleGeneric { # [suggestion (style = "short" , code = "" , applicability = "maybe-incorrect")] pub suggestion : Span , }
    };
}

BuiltinNoMangleGeneric!()