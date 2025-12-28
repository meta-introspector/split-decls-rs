macro_rules! deps {
    () => {
        Parser!();
        ValueParserFactory!();
        ValueParser!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl ValueParserFactory for std :: path :: PathBuf { type Parser = ValueParser ; fn value_parser () -> Self :: Parser { ValueParser :: path_buf () } }
    };
}

impl_329!();