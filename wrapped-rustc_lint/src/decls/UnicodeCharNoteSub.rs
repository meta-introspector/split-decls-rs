macro_rules! UnicodeCharNoteSub {
    () => {
        # [derive (Subdiagnostic)] # [label (lint_label_comment_char)] pub (crate) struct UnicodeCharNoteSub { # [primary_span] pub span : Span , pub c_debug : String , }
    };
}

UnicodeCharNoteSub!()