macro_rules! DanglingPointersFromTemporaries {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_dangling_pointers_from_temporaries)] # [note] # [help (lint_help_bind)] # [help (lint_help_returned)] # [help (lint_help_visit)] pub (crate) struct DanglingPointersFromTemporaries < 'tcx > { pub callee : Ident , pub ty : Ty < 'tcx > , # [label (lint_label_ptr)] pub ptr_span : Span , # [label (lint_label_temporary)] pub temporary_span : Span , }
    };
}

DanglingPointersFromTemporaries!();