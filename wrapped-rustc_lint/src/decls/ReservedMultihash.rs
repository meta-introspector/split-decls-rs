macro_rules! ReservedMultihash {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_reserved_multihash)] pub (crate) struct ReservedMultihash { # [suggestion (code = " " , applicability = "machine-applicable")] pub suggestion : Span , }
    };
}

ReservedMultihash!();