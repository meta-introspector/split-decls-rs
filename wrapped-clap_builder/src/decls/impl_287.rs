macro_rules! deps {
    () => {
        Arg!();
        PossibleValuesParser!();
        OsStr!();
        TypedValueParser!();
        PossibleValue!();
        Result!();
        Error!();
        Command!();
        Usage!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl TypedValueParser for PossibleValuesParser { type Value = String ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse (self , cmd , arg , value . to_owned ()) } fn parse (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < String , crate :: Error > { let value = ok ! (value . into_string () . map_err (| _ | { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; let ignore_case = arg . map (| a | a . is_ignore_case_set ()) . unwrap_or (false) ; if self . 0 . iter () . any (| v | v . matches (& value , ignore_case)) { Ok (value) } else { let possible_vals = self . 0 . iter () . filter (| v | ! v . is_hide_set ()) . map (| v | v . get_name () . to_owned ()) . collect :: < Vec < _ > > () ; Err (crate :: Error :: invalid_value (cmd , value , & possible_vals , arg . map (ToString :: to_string) . unwrap_or_else (| | "..." . to_owned ()) ,)) } } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { Some (Box :: new (self . 0 . iter () . cloned ())) } }
    };
}

impl_287!();