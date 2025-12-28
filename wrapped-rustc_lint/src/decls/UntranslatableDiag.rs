macro_rules! UntranslatableDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_untranslatable_diag)] pub (crate) struct UntranslatableDiag ;
    };
}

UntranslatableDiag!()