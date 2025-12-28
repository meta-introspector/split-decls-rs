macro_rules! deps {
    () => {
        RangedI64ValueParser!();
        ValueParserFactory!();
        Parser!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl ValueParserFactory for u32 { type Parser = RangedI64ValueParser < u32 > ; fn value_parser () -> Self :: Parser { let start : i64 = u32 :: MIN . into () ; let end : i64 = u32 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
    };
}

impl_336!();