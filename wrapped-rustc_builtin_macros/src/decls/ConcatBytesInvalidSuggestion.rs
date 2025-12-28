macro_rules! ConcatBytesInvalidSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ConcatBytesInvalidSuggestion { # [suggestion (builtin_macros_byte_char , code = "b{snippet}" , applicability = "machine-applicable")] CharLit { # [primary_span] span : Span , snippet : String , } , # [suggestion (builtin_macros_byte_str , code = "b{snippet}" , applicability = "machine-applicable")] StrLit { # [primary_span] span : Span , snippet : String , } , # [note (builtin_macros_c_str_note)] # [suggestion (builtin_macros_c_str , code = "{as_bstr}" , applicability = "machine-applicable")] CStrLit { # [primary_span] span : Span , as_bstr : String , } , # [suggestion (builtin_macros_number_array , code = "[{snippet}]" , applicability = "machine-applicable")] IntLit { # [primary_span] span : Span , snippet : String , } , }
    };
}

ConcatBytesInvalidSuggestion!();