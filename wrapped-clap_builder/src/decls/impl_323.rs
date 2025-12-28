macro_rules! deps {
    () => {
        StringValueParser!();
        OsStr!();
        Error!();
        ContextKind!();
        ContextValue!();
        ValueSource!();
        TypedValueParser!();
        UnknownArgumentValueParser!();
        Usage!();
        Arg!();
        Command!();
        Result!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl TypedValueParser for UnknownArgumentValueParser { type Value = String ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse_ref_ (self , cmd , arg , value , ValueSource :: CommandLine) } fn parse_ref_ (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , _value : & std :: ffi :: OsStr , source : ValueSource ,) -> Result < Self :: Value , crate :: Error > { match source { ValueSource :: DefaultValue => { TypedValueParser :: parse_ref_ (& StringValueParser :: new () , cmd , arg , _value , source) } ValueSource :: EnvVariable | ValueSource :: CommandLine => { let arg = match arg { Some (arg) => arg . to_string () , None => ".." . to_owned () , } ; let err = crate :: Error :: unknown_argument (cmd , arg , self . arg . as_ref () . map (| s | (s . as_str () . to_owned () , None)) , false , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) ; # [cfg (feature = "error-context")] let err = { debug_assert_eq ! (err . get (crate :: error :: ContextKind :: Suggested) , None , "Assuming `Error::unknown_argument` doesn't apply any `Suggested` so we can without caution") ; err . insert_context_unchecked (crate :: error :: ContextKind :: Suggested , crate :: error :: ContextValue :: StyledStrs (self . suggestions . clone ()) ,) } ; Err (err) } } } }
    };
}

impl_323!()