macro_rules! deps {
    () => {
        OsStr!();
        TypedValueParser!();
        Result!();
        Usage!();
        RangedI64ValueParser!();
        Error!();
        Command!();
        Arg!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < T : TryFrom < i64 > + Clone + Send + Sync + 'static > TypedValueParser for RangedI64ValueParser < T > where < T as TryFrom < i64 > > :: Error : Send + Sync + 'static + std :: error :: Error + ToString , { type Value = T ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , raw_value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (raw_value . to_str () . ok_or_else (|| { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; let value = ok ! (value . parse ::< i64 > () . map_err (| err | { let arg = arg . map (| a | a . to_string ()) . unwrap_or_else (|| "..." . to_owned ()) ; crate :: Error :: value_validation (arg , raw_value . to_string_lossy () . into_owned () , err . into () ,) . with_cmd (cmd) })) ; if ! self . bounds . contains (& value) { let arg = arg . map (| a | a . to_string ()) . unwrap_or_else (| | "..." . to_owned ()) ; return Err (crate :: Error :: value_validation (arg , raw_value . to_string_lossy () . into_owned () , format ! ("{} is not in {}" , value , self . format_bounds ()) . into () ,) . with_cmd (cmd)) ; } let value : Result < Self :: Value , _ > = value . try_into () ; let value = ok ! (value . map_err (| err | { let arg = arg . map (| a | a . to_string ()) . unwrap_or_else (|| "..." . to_owned ()) ; crate :: Error :: value_validation (arg , raw_value . to_string_lossy () . into_owned () , err . into () ,) . with_cmd (cmd) })) ; Ok (value) } }
    };
}

impl_291!()