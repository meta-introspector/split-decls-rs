macro_rules! MultipleDefaultsSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_suggestion , applicability = "maybe-incorrect" , style = "tool-only")] pub (crate) struct MultipleDefaultsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , pub (crate) ident : Ident , }
    };
}

MultipleDefaultsSugg!();