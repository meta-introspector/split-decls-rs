macro_rules! DestructorLabel {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_if_let_dtor)] struct DestructorLabel { # [primary_span] span : Span , dtor_kind : & 'static str , }
    };
}

DestructorLabel!()