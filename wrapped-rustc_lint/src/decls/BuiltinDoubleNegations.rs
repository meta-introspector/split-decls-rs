macro_rules! deps {
    () => {
        BuiltinDoubleNegationsAddParens!();
    };
}

macro_rules! BuiltinDoubleNegations {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_builtin_double_negations)] # [note (lint_note)] # [note (lint_note_decrement)] pub (crate) struct BuiltinDoubleNegations { # [subdiagnostic] pub add_parens : BuiltinDoubleNegationsAddParens , }
    };
}

BuiltinDoubleNegations!()