macro_rules! deps {
    () => {
        RangedI64ValueParser!();
        ValueParserFactory!();
        Parser!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl ValueParserFactory for i32 { type Parser = RangedI64ValueParser < i32 > ; fn value_parser () -> Self :: Parser { let start : i64 = i32 :: MIN . into () ; let end : i64 = i32 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
    };
}

impl_337!()