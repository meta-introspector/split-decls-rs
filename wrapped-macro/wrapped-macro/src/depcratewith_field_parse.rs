// Generated macro for with_field_parse (function)
macro_rules! Depcratewith_field_parse {
() => {
// Module: crate
// Provides: {"with_field_parse"}
// Dependencies: {}
fn with_field_parse (input : ParseStream < '_ >) -> Result < (String , WithOption) > { let interface = input . parse :: < syn :: LitStr > () ? . value () ; input . parse :: < Token ! [:] > () ? ; let start = input . span () ; let path = input . parse :: < syn :: Path > () ? ; let span = start . join (path . segments . last () . unwrap () . ident . span ()) . unwrap_or (start) ; if path . is_ident ("generate") { return Ok ((interface , WithOption :: Generate)) ; } let mut buf = String :: new () ; let append = | buf : & mut String , segment : syn :: PathSegment | -> Result < () > { if ! segment . arguments . is_none () { return Err (Error :: new (span , "Module path must not contain angles or parens" ,)) ; } buf . push_str (& segment . ident . to_string ()) ; Ok (()) } ; if path . leading_colon . is_some () { buf . push_str ("::") ; } let mut segments = path . segments . into_iter () ; if let Some (segment) = segments . next () { append (& mut buf , segment) ? ; } for segment in segments { buf . push_str ("::") ; append (& mut buf , segment) ? ; } Ok ((interface , WithOption :: Path (buf))) }
};
}
