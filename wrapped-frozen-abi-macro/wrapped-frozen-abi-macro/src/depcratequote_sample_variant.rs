// Generated macro for quote_sample_variant (function)
macro_rules! Depcratequote_sample_variant {
() => {
// Module: crate
// Provides: {"quote_sample_variant"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn quote_sample_variant (type_name : & Ident , ty_generics : & syn :: TypeGenerics , variant : & Variant ,) -> TokenStream2 { let variant_name = & variant . ident ; let variant = & variant . fields ; if * variant == Fields :: Unit { quote ! { let sample_variant : # type_name # ty_generics = # type_name ::# variant_name ; } } else if let Fields :: Unnamed (variant_fields) = variant { let mut fields = quote ! { } ; for field in & variant_fields . unnamed { if ! (field . ident . is_none () && field . colon_token . is_none ()) { unimplemented ! () ; } let ty = & field . ty ; fields . extend (quote ! { <# ty >:: example () , }) ; } quote ! { let sample_variant : # type_name # ty_generics = # type_name ::# variant_name (# fields) ; } } else if let Fields :: Named (variant_fields) = variant { let mut fields = quote ! { } ; for field in & variant_fields . named { if field . ident . is_none () || field . colon_token . is_none () { unimplemented ! () ; } let field_type_name = & field . ty ; let field_name = & field . ident ; fields . extend (quote ! { # field_name : <# field_type_name >:: example () , }) ; } quote ! { let sample_variant : # type_name # ty_generics = # type_name ::# variant_name { # fields } ; } } else { unimplemented ! ("variant: {:?}" , variant) } }
};
}
