macro_rules! deps {
    () => {
        RedundantSemicolonsSuggestion!();
    };
}

macro_rules! RedundantSemicolonsDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_redundant_semicolons)] pub (crate) struct RedundantSemicolonsDiag { pub multiple : bool , # [subdiagnostic] pub suggestion : Option < RedundantSemicolonsSuggestion > , }
    };
}

RedundantSemicolonsDiag!()