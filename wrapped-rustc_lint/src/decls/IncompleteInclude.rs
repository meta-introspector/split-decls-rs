macro_rules! IncompleteInclude {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_incomplete_include)] pub (crate) struct IncompleteInclude ;
    };
}

IncompleteInclude!()