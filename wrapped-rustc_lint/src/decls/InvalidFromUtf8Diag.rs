macro_rules! InvalidFromUtf8Diag {
    () => {
        # [derive (LintDiagnostic)] pub (crate) enum InvalidFromUtf8Diag { # [diag (lint_invalid_from_utf8_unchecked)] Unchecked { method : String , valid_up_to : usize , # [label] label : Span , } , # [diag (lint_invalid_from_utf8_checked)] Checked { method : String , valid_up_to : usize , # [label] label : Span , } , }
    };
}

InvalidFromUtf8Diag!()