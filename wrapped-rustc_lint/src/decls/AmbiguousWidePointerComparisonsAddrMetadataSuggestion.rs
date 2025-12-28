macro_rules! AmbiguousWidePointerComparisonsAddrMetadataSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_addr_metadata_suggestion , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousWidePointerComparisonsAddrMetadataSuggestion < 'a > { pub ne : & 'a str , pub deref_left : & 'a str , pub deref_right : & 'a str , pub l_modifiers : & 'a str , pub r_modifiers : & 'a str , # [suggestion_part (code = "{ne}std::ptr::eq({deref_left}")] pub left : Span , # [suggestion_part (code = "{l_modifiers}, {deref_right}")] pub middle : Span , # [suggestion_part (code = "{r_modifiers})")] pub right : Span , }
    };
}

AmbiguousWidePointerComparisonsAddrMetadataSuggestion!()