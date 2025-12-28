macro_rules! TrailingMacro {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_trailing_semi_macro)] pub (crate) struct TrailingMacro { # [note (lint_note1)] # [note (lint_note2)] pub is_trailing : bool , pub name : Ident , }
    };
}

TrailingMacro!()