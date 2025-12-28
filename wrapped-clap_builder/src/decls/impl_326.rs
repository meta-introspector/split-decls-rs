macro_rules! deps {
    () => {
        ValueParserFactory!();
        StringValueParser!();
        Parser!();
        MapValueParser!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl ValueParserFactory for Box < str > { type Parser = MapValueParser < StringValueParser , fn (String) -> Box < str > > ; fn value_parser () -> Self :: Parser { StringValueParser :: new () . map (String :: into_boxed_str) } }
    };
}

impl_326!();