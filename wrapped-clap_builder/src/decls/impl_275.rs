macro_rules! deps {
    () => {
        Result!();
        Command!();
        TypedValueParser!();
        Error!();
        OsStr!();
        OsStringValueParser!();
        Arg!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl TypedValueParser for OsStringValueParser { type Value = std :: ffi :: OsString ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse (self , cmd , arg , value . to_owned ()) } fn parse (& self , _cmd : & crate :: Command , _arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { Ok (value) } }
    };
}

impl_275!();