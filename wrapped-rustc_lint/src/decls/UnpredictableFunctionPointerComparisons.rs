macro_rules! deps {
    () => {
        UnpredictableFunctionPointerComparisonsSuggestion!();
    };
}

macro_rules! UnpredictableFunctionPointerComparisons {
    () => {
        deps!();
        # [derive (LintDiagnostic)] pub (crate) enum UnpredictableFunctionPointerComparisons < 'a , 'tcx > { # [diag (lint_unpredictable_fn_pointer_comparisons)] # [note (lint_note_duplicated_fn)] # [note (lint_note_deduplicated_fn)] # [note (lint_note_visit_fn_addr_eq)] Suggestion { # [subdiagnostic] sugg : UnpredictableFunctionPointerComparisonsSuggestion < 'a , 'tcx > , } , # [diag (lint_unpredictable_fn_pointer_comparisons)] # [note (lint_note_duplicated_fn)] # [note (lint_note_deduplicated_fn)] # [note (lint_note_visit_fn_addr_eq)] Warn , }
    };
}

UnpredictableFunctionPointerComparisons!()