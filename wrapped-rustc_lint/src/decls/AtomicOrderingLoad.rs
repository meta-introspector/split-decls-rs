macro_rules! AtomicOrderingLoad {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_atomic_ordering_load)] # [help] pub (crate) struct AtomicOrderingLoad ;
    };
}

AtomicOrderingLoad!();