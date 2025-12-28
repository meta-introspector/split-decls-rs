macro_rules! deps {
    () => {
        MapValueParser!();
        OsStr!();
        Arg!();
        Result!();
        Error!();
        Command!();
        PossibleValue!();
        TypedValueParser!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < P , F , T > TypedValueParser for MapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> T + Clone + Send + Sync + 'static , T : Send + Sync + Clone , { type Value = T ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (self . parser . parse_ref (cmd , arg , value)) ; let value = (self . func) (value) ; Ok (value) } fn parse (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (self . parser . parse (cmd , arg , value)) ; let value = (self . func) (value) ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { self . parser . possible_values () } }
    };
}

impl_317!();