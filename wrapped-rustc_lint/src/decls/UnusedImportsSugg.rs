macro_rules! UnusedImportsSugg {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnusedImportsSugg { # [suggestion (lint_suggestion_remove_whole_use , applicability = "machine-applicable" , code = "" , style = "tool-only")] RemoveWholeUse { # [primary_span] span : Span , } , # [multipart_suggestion (lint_suggestion_remove_imports , applicability = "machine-applicable" , style = "tool-only")] RemoveImports { # [suggestion_part (code = "")] remove_spans : Vec < Span > , num_to_remove : usize , } , }
    };
}

UnusedImportsSugg!();