macro_rules! deps {
    () => {
        PropertyUnicodeSet!();
    };
}

macro_rules! EmojiSetDataBorrowed {
    () => {
        deps!();
        # [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`EmojiSetData::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct EmojiSetDataBorrowed < 'a > { set : & 'a PropertyUnicodeSet < 'a > , }
    };
}

EmojiSetDataBorrowed!()