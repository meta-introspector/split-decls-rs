macro_rules! InvalidNullArgumentsDiag {
    () => {
        # [derive (LintDiagnostic)] pub (crate) enum InvalidNullArgumentsDiag { # [diag (lint_invalid_null_arguments)] # [help (lint_doc)] NullPtrInline { # [label (lint_origin)] null_span : Span , } , # [diag (lint_invalid_null_arguments)] # [help (lint_doc)] NullPtrThroughBinding { # [note (lint_origin)] null_span : Span , } , }
    };
}

InvalidNullArgumentsDiag!()