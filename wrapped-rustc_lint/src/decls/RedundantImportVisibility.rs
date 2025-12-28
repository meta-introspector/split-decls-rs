macro_rules! RedundantImportVisibility {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_redundant_import_visibility)] pub (crate) struct RedundantImportVisibility { # [note] pub span : Span , # [help] pub help : () , pub import_vis : String , pub max_vis : String , }
    };
}

RedundantImportVisibility!()