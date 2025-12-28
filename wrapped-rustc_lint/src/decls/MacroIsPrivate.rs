macro_rules! MacroIsPrivate {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_macro_is_private)] pub (crate) struct MacroIsPrivate { pub ident : Ident , }
    };
}

MacroIsPrivate!();