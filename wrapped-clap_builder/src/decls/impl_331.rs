macro_rules! deps {
    () => {
        ValueParser!();
        Parser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl ValueParserFactory for bool { type Parser = ValueParser ; fn value_parser () -> Self :: Parser { ValueParser :: bool () } }
    };
}

impl_331!()