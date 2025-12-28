macro_rules! deps {
    () => {
        Parser!();
        RangedI64ValueParser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl ValueParserFactory for i64 { type Parser = RangedI64ValueParser < i64 > ; fn value_parser () -> Self :: Parser { RangedI64ValueParser :: new () } }
    };
}

impl_339!()