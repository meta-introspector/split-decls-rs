macro_rules! UnusedImportBracesDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_import_braces)] pub (crate) struct UnusedImportBracesDiag { pub node : Symbol , }
    };
}

UnusedImportBracesDiag!();