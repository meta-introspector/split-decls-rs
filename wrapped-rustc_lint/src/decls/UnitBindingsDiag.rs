macro_rules! UnitBindingsDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unit_bindings)] pub (crate) struct UnitBindingsDiag { # [label] pub label : Span , }
    };
}

UnitBindingsDiag!()