macro_rules! deps {
    () => {
        Parser!();
        ValueParserFactory!();
        RangedI64ValueParser!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl ValueParserFactory for u8 { type Parser = RangedI64ValueParser < u8 > ; fn value_parser () -> Self :: Parser { let start : i64 = u8 :: MIN . into () ; let end : i64 = u8 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
    };
}

impl_332!()