macro_rules! HiddenGlobReexports {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_hidden_glob_reexport)] pub (crate) struct HiddenGlobReexports { # [note (lint_note_glob_reexport)] pub glob_reexport : Span , # [note (lint_note_private_item)] pub private_item : Span , pub name : String , pub namespace : String , }
    };
}

HiddenGlobReexports!();