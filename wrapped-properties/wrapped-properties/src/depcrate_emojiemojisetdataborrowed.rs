// Generated macro for EmojiSetDataBorrowed (struct)
macro_rules! Depcrate_emojiEmojiSetDataBorrowed {
() => {
// Module: crate::emoji
// Provides: {"EmojiSetDataBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`EmojiSetData::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct EmojiSetDataBorrowed < 'a > { set : & 'a PropertyUnicodeSet < 'a > , }
};
}
