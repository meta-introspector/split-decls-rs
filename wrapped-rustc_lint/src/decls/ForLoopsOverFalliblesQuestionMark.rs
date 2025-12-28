macro_rules! ForLoopsOverFalliblesQuestionMark {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_use_question_mark , code = "?" , applicability = "maybe-incorrect")] pub (crate) struct ForLoopsOverFalliblesQuestionMark { # [primary_span] pub suggestion : Span , }
    };
}

ForLoopsOverFalliblesQuestionMark!()