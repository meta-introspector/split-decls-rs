macro_rules! deps {
    () => {
        TypedValueParser!();
        MapValueParser!();
        Parser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < T > ValueParserFactory for Box < T > where T : ValueParserFactory , < T as ValueParserFactory > :: Parser : TypedValueParser < Value = T > , T : Send + Sync + Clone , { type Parser = MapValueParser < < T as ValueParserFactory > :: Parser , fn (T) -> Box < T > > ; fn value_parser () -> Self :: Parser { T :: value_parser () . map (Box :: new) } }
    };
}

impl_342!()