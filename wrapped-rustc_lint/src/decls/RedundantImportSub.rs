macro_rules! RedundantImportSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum RedundantImportSub { # [label (lint_label_imported_here)] ImportedHere (# [primary_span] Span) , # [label (lint_label_defined_here)] DefinedHere (# [primary_span] Span) , # [label (lint_label_imported_prelude)] ImportedPrelude (# [primary_span] Span) , # [label (lint_label_defined_prelude)] DefinedPrelude (# [primary_span] Span) , }
    };
}

RedundantImportSub!();