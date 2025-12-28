macro_rules! deps {
    () => {
        SentenceBreak!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        impl_value_getter ! { impl SentenceBreak { PropertyNameParseSentenceBreakV1 / SINGLETON_PROPERTY_NAME_PARSE_SENTENCE_BREAK_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortSentenceBreakV1 / SINGLETON_PROPERTY_NAME_SHORT_SENTENCE_BREAK_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongSentenceBreakV1 / SINGLETON_PROPERTY_NAME_LONG_SENTENCE_BREAK_V1 ; } }
    };
}

macro_71!();