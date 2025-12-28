macro_rules! deps {
    () => {
        PathBufValueParser!();
        Command!();
        OsStr!();
        TypedValueParser!();
        Error!();
        Result!();
        Arg!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl TypedValueParser for PathBufValueParser { type Value = std :: path :: PathBuf ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse (self , cmd , arg , value . to_owned ()) } fn parse (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { if value . is_empty () { return Err (crate :: Error :: empty_value (cmd , & [] , arg . map (ToString :: to_string) . unwrap_or_else (| | "..." . to_owned ()) ,)) ; } Ok (Self :: Value :: from (value)) } }
    };
}

impl_279!();