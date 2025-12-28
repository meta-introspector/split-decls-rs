macro_rules! AtomicOrderingFence {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_atomic_ordering_fence)] # [help] pub (crate) struct AtomicOrderingFence ;
    };
}

AtomicOrderingFence!()