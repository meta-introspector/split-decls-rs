macro_rules! deps {
    () => {
        UnknownDiagnosticAttributeTypoSugg!();
    };
}

macro_rules! UnknownDiagnosticAttribute {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unknown_diagnostic_attribute)] pub (crate) struct UnknownDiagnosticAttribute { # [subdiagnostic] pub typo : Option < UnknownDiagnosticAttributeTypoSugg > , }
    };
}

UnknownDiagnosticAttribute!();