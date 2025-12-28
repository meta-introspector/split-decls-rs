macro_rules! deps {
    () => {
        OsStr!();
        TypedValueParser!();
        Usage!();
        Error!();
        Arg!();
        Command!();
        Result!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < F , T , E > TypedValueParser for F where F : Fn (& str) -> Result < T , E > + Clone + Send + Sync + 'static , E : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , T : Send + Sync + Clone , { type Value = T ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (value . to_str () . ok_or_else (|| { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; let value = ok ! ((self) (value) . map_err (| e | { let arg = arg . map (| a | a . to_string ()) . unwrap_or_else (|| "..." . to_owned ()) ; crate :: Error :: value_validation (arg , value . to_owned () , e . into ()) . with_cmd (cmd) })) ; Ok (value) } }
    };
}

impl_268!();