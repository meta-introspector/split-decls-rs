macro_rules! BuiltinExplicitOutlivesSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion)] pub (crate) struct BuiltinExplicitOutlivesSuggestion { # [suggestion_part (code = "")] pub spans : Vec < Span > , # [applicability] pub applicability : Applicability , }
    };
}

BuiltinExplicitOutlivesSuggestion!()