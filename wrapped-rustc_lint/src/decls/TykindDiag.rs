macro_rules! TykindDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_tykind)] # [help] pub (crate) struct TykindDiag ;
    };
}

TykindDiag!();