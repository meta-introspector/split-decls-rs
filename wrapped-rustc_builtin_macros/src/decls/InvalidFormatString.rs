macro_rules! deps {
    () => {
        InvalidFormatStringNote!();
        InvalidFormatStringLabel!();
        InvalidFormatStringSuggestion!();
    };
}

macro_rules! InvalidFormatString {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_format_string_invalid)] pub (crate) struct InvalidFormatString { # [primary_span] # [label] pub (crate) span : Span , pub (crate) desc : String , pub (crate) label1 : String , # [subdiagnostic] pub (crate) note_ : Option < InvalidFormatStringNote > , # [subdiagnostic] pub (crate) label_ : Option < InvalidFormatStringLabel > , # [subdiagnostic] pub (crate) sugg_ : Option < InvalidFormatStringSuggestion > , }
    };
}

InvalidFormatString!()