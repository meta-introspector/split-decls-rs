macro_rules! UnqualifiedLocalImportsDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unqualified_local_imports)] pub (crate) struct UnqualifiedLocalImportsDiag { }
    };
}

UnqualifiedLocalImportsDiag!();