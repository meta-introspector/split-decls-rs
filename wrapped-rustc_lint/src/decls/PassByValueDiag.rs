macro_rules! PassByValueDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_pass_by_value)] pub (crate) struct PassByValueDiag { pub ty : String , # [suggestion (code = "{ty}" , applicability = "maybe-incorrect")] pub suggestion : Span , }
    };
}

PassByValueDiag!();