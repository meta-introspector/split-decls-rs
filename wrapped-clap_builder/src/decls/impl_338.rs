macro_rules! deps {
    () => {
        RangedU64ValueParser!();
        ValueParserFactory!();
        Parser!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl ValueParserFactory for u64 { type Parser = RangedU64ValueParser < u64 > ; fn value_parser () -> Self :: Parser { RangedU64ValueParser :: new () } }
    };
}

impl_338!();