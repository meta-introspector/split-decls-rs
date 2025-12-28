macro_rules! MultipleDefaultAttrsSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_help , applicability = "machine-applicable" , style = "tool-only")] pub (crate) struct MultipleDefaultAttrsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , }
    };
}

MultipleDefaultAttrsSugg!();