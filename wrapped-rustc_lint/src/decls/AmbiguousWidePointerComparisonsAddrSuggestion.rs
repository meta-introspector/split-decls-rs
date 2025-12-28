macro_rules! AmbiguousWidePointerComparisonsAddrSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_addr_suggestion , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousWidePointerComparisonsAddrSuggestion < 'a > { pub (crate) ne : & 'a str , pub (crate) deref_left : & 'a str , pub (crate) deref_right : & 'a str , pub (crate) l_modifiers : & 'a str , pub (crate) r_modifiers : & 'a str , # [suggestion_part (code = "{ne}std::ptr::addr_eq({deref_left}")] pub (crate) left : Span , # [suggestion_part (code = "{l_modifiers}, {deref_right}")] pub (crate) middle : Span , # [suggestion_part (code = "{r_modifiers})")] pub (crate) right : Span , }
    };
}

AmbiguousWidePointerComparisonsAddrSuggestion!()