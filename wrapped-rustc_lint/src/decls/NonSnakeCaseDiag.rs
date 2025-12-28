macro_rules! deps {
    () => {
        NonSnakeCaseDiagSub!();
    };
}

macro_rules! NonSnakeCaseDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_non_snake_case)] pub (crate) struct NonSnakeCaseDiag < 'a > { pub sort : & 'a str , pub name : & 'a str , pub sc : String , # [subdiagnostic] pub sub : NonSnakeCaseDiagSub , }
    };
}

NonSnakeCaseDiag!();