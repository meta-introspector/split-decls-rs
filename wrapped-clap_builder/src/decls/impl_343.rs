macro_rules! deps {
    () => {
        ValueParserFactory!();
        Parser!();
        MapValueParser!();
        TypedValueParser!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < T > ValueParserFactory for std :: sync :: Arc < T > where T : ValueParserFactory , < T as ValueParserFactory > :: Parser : TypedValueParser < Value = T > , T : Send + Sync + Clone , { type Parser = MapValueParser < < T as ValueParserFactory > :: Parser , fn (T) -> std :: sync :: Arc < T > > ; fn value_parser () -> Self :: Parser { T :: value_parser () . map (std :: sync :: Arc :: new) } }
    };
}

impl_343!()