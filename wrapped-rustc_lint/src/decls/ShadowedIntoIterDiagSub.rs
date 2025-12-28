macro_rules! ShadowedIntoIterDiagSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ShadowedIntoIterDiagSub { # [suggestion (lint_remove_into_iter_suggestion , code = "" , applicability = "maybe-incorrect")] RemoveIntoIter { # [primary_span] span : Span , } , # [multipart_suggestion (lint_use_explicit_into_iter_suggestion , applicability = "maybe-incorrect")] UseExplicitIntoIter { # [suggestion_part (code = "IntoIterator::into_iter(")] start_span : Span , # [suggestion_part (code = ")")] end_span : Span , } , }
    };
}

ShadowedIntoIterDiagSub!()