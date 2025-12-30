// Generated macro for get (function)
macro_rules! Depcrate_attrget {
() => {
// Module: crate::attr
// Provides: {"get"}
// Dependencies: {}
pub fn get (input : & [Attribute]) -> Result < Attrs > { let mut attrs = Attrs { display : None , source : None , backtrace : None , from : None , transparent : None , fmt : None , } ; for attr in input { if attr . path () . is_ident ("error") { parse_error_attribute (& mut attrs , attr) ? ; } else if attr . path () . is_ident ("source") { attr . meta . require_path_only () ? ; if attrs . source . is_some () { return Err (Error :: new_spanned (attr , "duplicate #[source] attribute")) ; } let span = (attr . pound_token . span) . join (attr . bracket_token . span . join ()) . unwrap_or (attr . path () . get_ident () . unwrap () . span ()) ; attrs . source = Some (Source { original : attr , span , }) ; } else if attr . path () . is_ident ("backtrace") { attr . meta . require_path_only () ? ; if attrs . backtrace . is_some () { return Err (Error :: new_spanned (attr , "duplicate #[backtrace] attribute")) ; } attrs . backtrace = Some (attr) ; } else if attr . path () . is_ident ("from") { match attr . meta { Meta :: Path (_) => { } Meta :: List (_) | Meta :: NameValue (_) => { continue ; } } if attrs . from . is_some () { return Err (Error :: new_spanned (attr , "duplicate #[from] attribute")) ; } let span = (attr . pound_token . span) . join (attr . bracket_token . span . join ()) . unwrap_or (attr . path () . get_ident () . unwrap () . span ()) ; attrs . from = Some (From { original : attr , span , }) ; } } Ok (attrs) }
};
}
