macro_rules! deps {
    () => {
        OsStr!();
        EnumValueParser!();
        ValueEnum!();
        Arg!();
        PossibleValue!();
        Result!();
        Error!();
        Command!();
        TypedValueParser!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < E : crate :: ValueEnum + Clone + Send + Sync + 'static > TypedValueParser for EnumValueParser < E > { type Value = E ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let ignore_case = arg . map (| a | a . is_ignore_case_set ()) . unwrap_or (false) ; let possible_vals = | | { E :: value_variants () . iter () . filter_map (| v | v . to_possible_value ()) . filter (| v | ! v . is_hide_set ()) . map (| v | v . get_name () . to_owned ()) . collect :: < Vec < _ > > () } ; let value = ok ! (value . to_str () . ok_or_else (|| { crate :: Error :: invalid_value (cmd , value . to_string_lossy () . into_owned () , & possible_vals () , arg . map (ToString :: to_string) . unwrap_or_else (|| "..." . to_owned ()) ,) })) ; let value = ok ! (E :: value_variants () . iter () . find (| v | { v . to_possible_value () . expect ("ValueEnum::value_variants contains only values with a corresponding ValueEnum::to_possible_value") . matches (value , ignore_case) }) . ok_or_else (|| { crate :: Error :: invalid_value (cmd , value . to_owned () , & possible_vals () , arg . map (ToString :: to_string) . unwrap_or_else (|| "..." . to_owned ()) ,) })) . clone () ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { Some (Box :: new (E :: value_variants () . iter () . filter_map (| v | v . to_possible_value ()) ,)) } }
    };
}

impl_283!()