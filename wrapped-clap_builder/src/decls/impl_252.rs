macro_rules! deps {
    () => {
        PossibleValue!();
        AnyValueParser!();
        ValueParser!();
        OsStringValueParser!();
        ValueSource!();
        Result!();
        OsStr!();
        Command!();
        AnyValue!();
        Error!();
        PathBufValueParser!();
        StringValueParser!();
        AnyValueId!();
        ValueParserInner!();
        Arg!();
        BoolValueParser!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl ValueParser { # [doc = " Parse into a `AnyValue`"] # [doc = ""] # [doc = " When `arg` is `None`, an external subcommand value is being parsed."] pub (crate) fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr , source : ValueSource ,) -> Result < AnyValue , crate :: Error > { self . any_value_parser () . parse_ref_ (cmd , arg , value , source) } # [doc = " Describes the content of `AnyValue`"] pub fn type_id (& self) -> AnyValueId { self . any_value_parser () . type_id () } # [doc = " Reflect on enumerated value properties"] # [doc = ""] # [doc = " Error checking should not be done with this; it is mostly targeted at user-facing"] # [doc = " applications like errors and completion."] pub fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { self . any_value_parser () . possible_values () } fn any_value_parser (& self) -> & dyn AnyValueParser { match & self . 0 { ValueParserInner :: Bool => & BoolValueParser { } , ValueParserInner :: String => & StringValueParser { } , ValueParserInner :: OsString => & OsStringValueParser { } , ValueParserInner :: PathBuf => & PathBufValueParser { } , ValueParserInner :: Other (o) => o . as_ref () , } } }
    };
}

impl_252!();