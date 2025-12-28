macro_rules! SuspiciousDoubleRefDerefDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_suspicious_double_ref_deref)] pub (crate) struct SuspiciousDoubleRefDerefDiag < 'a > { pub ty : Ty < 'a > , }
    };
}

SuspiciousDoubleRefDerefDiag!();