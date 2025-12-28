macro_rules! deps {
    () => {
        ValueParserFactory!();
        Parser!();
        ValueParser!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl ValueParserFactory for std :: ffi :: OsString { type Parser = ValueParser ; fn value_parser () -> Self :: Parser { ValueParser :: os_string () } }
    };
}

impl_327!();