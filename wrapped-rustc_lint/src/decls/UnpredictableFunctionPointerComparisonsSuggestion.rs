macro_rules! UnpredictableFunctionPointerComparisonsSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnpredictableFunctionPointerComparisonsSuggestion < 'a , 'tcx > { # [multipart_suggestion (lint_fn_addr_eq_suggestion , style = "verbose" , applicability = "maybe-incorrect")] FnAddrEq { ne : & 'a str , deref_left : & 'a str , deref_right : & 'a str , # [suggestion_part (code = "{ne}std::ptr::fn_addr_eq({deref_left}")] left : Span , # [suggestion_part (code = ", {deref_right}")] middle : Span , # [suggestion_part (code = ")")] right : Span , } , # [multipart_suggestion (lint_fn_addr_eq_suggestion , style = "verbose" , applicability = "maybe-incorrect")] FnAddrEqWithCast { ne : & 'a str , deref_left : & 'a str , deref_right : & 'a str , fn_sig : rustc_middle :: ty :: PolyFnSig < 'tcx > , # [suggestion_part (code = "{ne}std::ptr::fn_addr_eq({deref_left}")] left : Span , # [suggestion_part (code = ", {deref_right}")] middle : Span , # [suggestion_part (code = " as {fn_sig})")] right : Span , } , }
    };
}

UnpredictableFunctionPointerComparisonsSuggestion!();