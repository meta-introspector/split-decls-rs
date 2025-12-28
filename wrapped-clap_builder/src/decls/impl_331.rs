macro_rules! deps {
    () => {
        Parser!();
        ValueParser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl ValueParserFactory for bool { type Parser = ValueParser ; fn value_parser () -> Self :: Parser { ValueParser :: bool () } }
    };
}

impl_331!();