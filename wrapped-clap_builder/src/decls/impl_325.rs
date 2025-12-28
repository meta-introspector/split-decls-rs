macro_rules! deps {
    () => {
        ValueParser!();
        ValueParserFactory!();
        Parser!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl ValueParserFactory for String { type Parser = ValueParser ; fn value_parser () -> Self :: Parser { ValueParser :: string () } }
    };
}

impl_325!()