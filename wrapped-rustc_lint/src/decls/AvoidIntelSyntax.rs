macro_rules! AvoidIntelSyntax {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_avoid_intel_syntax)] pub (crate) struct AvoidIntelSyntax ;
    };
}

AvoidIntelSyntax!()