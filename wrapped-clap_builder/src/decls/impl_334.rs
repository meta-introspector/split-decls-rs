macro_rules! deps {
    () => {
        RangedI64ValueParser!();
        Parser!();
        ValueParserFactory!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl ValueParserFactory for u16 { type Parser = RangedI64ValueParser < u16 > ; fn value_parser () -> Self :: Parser { let start : i64 = u16 :: MIN . into () ; let end : i64 = u16 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
    };
}

impl_334!()