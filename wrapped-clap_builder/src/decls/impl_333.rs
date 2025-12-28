macro_rules! deps {
    () => {
        Parser!();
        RangedI64ValueParser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl ValueParserFactory for i8 { type Parser = RangedI64ValueParser < i8 > ; fn value_parser () -> Self :: Parser { let start : i64 = i8 :: MIN . into () ; let end : i64 = i8 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
    };
}

impl_333!();