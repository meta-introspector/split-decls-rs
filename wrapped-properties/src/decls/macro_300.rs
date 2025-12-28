macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_300 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryEmojiPresentationV1`"] PropertyBinaryEmojiPresentationV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_300!()