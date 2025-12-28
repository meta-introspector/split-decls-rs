macro_rules! deps {
    () => {
        MapValueParser!();
        PathBufValueParser!();
        ValueParserFactory!();
        Parser!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl ValueParserFactory for Box < std :: path :: Path > { type Parser = MapValueParser < PathBufValueParser , fn (std :: path :: PathBuf) -> Box < std :: path :: Path > > ; fn value_parser () -> Self :: Parser { PathBufValueParser :: new () . map (std :: path :: PathBuf :: into_boxed_path) } }
    };
}

impl_330!();