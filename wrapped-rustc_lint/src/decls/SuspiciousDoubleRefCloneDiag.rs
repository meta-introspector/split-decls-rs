macro_rules! SuspiciousDoubleRefCloneDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_suspicious_double_ref_clone)] pub (crate) struct SuspiciousDoubleRefCloneDiag < 'a > { pub ty : Ty < 'a > , }
    };
}

SuspiciousDoubleRefCloneDiag!();