macro_rules! deps {
    () => {
        MapValueParser!();
        ValueParserFactory!();
        TypedValueParser!();
        Parser!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl < T > ValueParserFactory for std :: num :: Saturating < T > where T : ValueParserFactory , < T as ValueParserFactory > :: Parser : TypedValueParser < Value = T > , T : Send + Sync + Clone , { type Parser = MapValueParser < < T as ValueParserFactory > :: Parser , fn (T) -> std :: num :: Saturating < T > > ; fn value_parser () -> Self :: Parser { T :: value_parser () . map (std :: num :: Saturating) } }
    };
}

impl_340!()