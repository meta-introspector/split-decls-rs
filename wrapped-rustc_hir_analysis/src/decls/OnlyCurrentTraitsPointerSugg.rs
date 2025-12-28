macro_rules! OnlyCurrentTraitsPointerSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_analysis_only_current_traits_pointer_sugg , applicability = "maybe-incorrect")] pub (crate) struct OnlyCurrentTraitsPointerSugg < 'a > { # [suggestion_part (code = "WrapperType")] pub wrapper_span : Span , # [suggestion_part (code = "struct WrapperType(*{mut_key}{ptr_ty});\n\n")] pub (crate) struct_span : Span , pub mut_key : & 'a str , pub ptr_ty : Ty < 'a > , }
    };
}

OnlyCurrentTraitsPointerSugg!()