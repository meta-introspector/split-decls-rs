macro_rules! deps {
    () => {
        PropertyUnicodeSet!();
    };
}

macro_rules! EmojiSetData {
    () => {
        deps!();
        # [doc = " A wrapper around `UnicodeSet` data (characters and strings)"] # [derive (Debug)] pub struct EmojiSetData { data : DataPayload < ErasedMarker < PropertyUnicodeSet < 'static > > > , }
    };
}

EmojiSetData!()