macro_rules! SingleUseLifetimeSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "machine-applicable")] pub (crate) struct SingleUseLifetimeSugg { # [suggestion_part (code = "")] pub deletion_span : Option < Span > , # [suggestion_part (code = "{replace_lt}")] pub use_span : Span , pub replace_lt : String , }
    };
}

SingleUseLifetimeSugg!()