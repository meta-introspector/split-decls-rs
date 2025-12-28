macro_rules! deps {
    () => {
        ValueParserFactory!();
        OsStr!();
        MapValueParser!();
        Parser!();
        OsStringValueParser!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl ValueParserFactory for Box < std :: ffi :: OsStr > { type Parser = MapValueParser < OsStringValueParser , fn (std :: ffi :: OsString) -> Box < std :: ffi :: OsStr > > ; fn value_parser () -> Self :: Parser { OsStringValueParser :: new () . map (std :: ffi :: OsString :: into_boxed_os_str) } }
    };
}

impl_328!();