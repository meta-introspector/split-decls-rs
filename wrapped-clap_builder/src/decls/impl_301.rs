macro_rules! deps {
    () => {
        PossibleValue!();
        Result!();
        BoolValueParser!();
        Command!();
        Arg!();
        TypedValueParser!();
        OsStr!();
        Error!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl TypedValueParser for BoolValueParser { type Value = bool ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = if value == std :: ffi :: OsStr :: new ("true") { true } else if value == std :: ffi :: OsStr :: new ("false") { false } else { let possible_vals = Self :: possible_values () . map (| v | v . get_name () . to_owned ()) . collect :: < Vec < _ > > () ; return Err (crate :: Error :: invalid_value (cmd , value . to_string_lossy () . into_owned () , & possible_vals , arg . map (ToString :: to_string) . unwrap_or_else (| | "..." . to_owned ()) ,)) ; } ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { Some (Box :: new (Self :: possible_values ())) } }
    };
}

impl_301!();