macro_rules! LintPassByHand {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_lintpass_by_hand)] # [help] pub (crate) struct LintPassByHand ;
    };
}

LintPassByHand!();