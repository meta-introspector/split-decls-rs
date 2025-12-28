macro_rules! UnicodeTextFlowSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "machine-applicable" , style = "hidden")] pub (crate) struct UnicodeTextFlowSuggestion { # [suggestion_part (code = "")] pub spans : Vec < Span > , }
    };
}

UnicodeTextFlowSuggestion!()