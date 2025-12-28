macro_rules! AvoidAttSyntax {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_avoid_att_syntax)] pub (crate) struct AvoidAttSyntax ;
    };
}

AvoidAttSyntax!()