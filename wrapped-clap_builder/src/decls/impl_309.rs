macro_rules! deps {
    () => {
        TypedValueParser!();
        Command!();
        Usage!();
        Error!();
        Result!();
        PossibleValue!();
        OsStr!();
        Arg!();
        BoolishValueParser!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl TypedValueParser for BoolishValueParser { type Value = bool ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (value . to_str () . ok_or_else (|| { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; let value = ok ! (crate :: util :: str_to_bool (value) . ok_or_else (|| { let arg = arg . map (| a | a . to_string ()) . unwrap_or_else (|| "..." . to_owned ()) ; crate :: Error :: value_validation (arg , value . to_owned () , "value was not a boolean" . into ()) . with_cmd (cmd) })) ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { Some (Box :: new (Self :: possible_values ())) } }
    };
}

impl_309!()