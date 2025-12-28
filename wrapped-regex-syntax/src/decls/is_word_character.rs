macro_rules! is_word_character {
    () => {
        # [doc = " Returns true if and only if the given character is a Unicode word"] # [doc = " character."] # [doc = ""] # [doc = " A Unicode word character is defined by"] # [doc = " [UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties)."] # [doc = " In particular, a character"] # [doc = " is considered a word character if it is in either of the `Alphabetic` or"] # [doc = " `Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`"] # [doc = " or `Connector_Punctuation` general categories."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the `unicode-perl` feature is not enabled, then this function"] # [doc = " panics. For this reason, it is recommended that callers use"] # [doc = " [`try_is_word_character`] instead."] pub fn is_word_character (c : char) -> bool { try_is_word_character (c) . expect ("unicode-perl feature must be enabled") }
    };
}

is_word_character!();