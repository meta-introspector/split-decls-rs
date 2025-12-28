macro_rules! InvalidAtomicOrderingDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_atomic_ordering_invalid)] # [help] pub (crate) struct InvalidAtomicOrderingDiag { pub method : Symbol , # [label] pub fail_order_arg_span : Span , }
    };
}

InvalidAtomicOrderingDiag!();