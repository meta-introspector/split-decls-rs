macro_rules! deps {
    () => {
        BreakWithLabelAndLoopSub!();
    };
}

macro_rules! BreakWithLabelAndLoop {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_break_with_label_and_loop)] pub (crate) struct BreakWithLabelAndLoop { # [subdiagnostic] pub sub : BreakWithLabelAndLoopSub , }
    };
}

BreakWithLabelAndLoop!();