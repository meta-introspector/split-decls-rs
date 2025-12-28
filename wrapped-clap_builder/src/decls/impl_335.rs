macro_rules! deps {
    () => {
        Parser!();
        RangedI64ValueParser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl ValueParserFactory for i16 { type Parser = RangedI64ValueParser < i16 > ; fn value_parser () -> Self :: Parser { let start : i64 = i16 :: MIN . into () ; let end : i64 = i16 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
    };
}

impl_335!();