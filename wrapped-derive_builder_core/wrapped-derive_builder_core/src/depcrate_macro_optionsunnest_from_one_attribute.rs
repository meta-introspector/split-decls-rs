// Generated macro for unnest_from_one_attribute (function)
macro_rules! Depcrate_macro_optionsunnest_from_one_attribute {
() => {
// Module: crate::macro_options
// Provides: {"unnest_from_one_attribute"}
// Dependencies: {}
fn unnest_from_one_attribute (attr : syn :: Attribute) -> darling :: Result < Attribute > { match & attr . style { syn :: AttrStyle :: Outer => () , syn :: AttrStyle :: Inner (bang) => { return Err (darling :: Error :: unsupported_format (& format ! ("{} must be an outer attribute" , attr . path () . get_ident () . map (Ident :: to_string) . unwrap_or_else (|| "Attribute" . to_string ()))) . with_span (bang)) ; } } ; let original_span = attr . span () ; let pound = attr . pound_token ; let meta = attr . meta ; match meta { Meta :: Path (_) => Err (Error :: unsupported_format ("word") . with_span (& meta)) , Meta :: NameValue (_) => Err (Error :: unsupported_format ("name-value") . with_span (& meta)) , Meta :: List (list) => { let inner = list . tokens ; Ok (parse_quote_spanned ! (original_span => # pound [# inner])) } } }
};
}
