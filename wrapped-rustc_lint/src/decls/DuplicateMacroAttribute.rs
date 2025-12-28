macro_rules! DuplicateMacroAttribute {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_duplicate_macro_attribute)] pub (crate) struct DuplicateMacroAttribute ;
    };
}

DuplicateMacroAttribute!();