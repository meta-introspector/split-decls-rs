macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_301 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryEmojiV1`"] PropertyBinaryEmojiV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_301!();