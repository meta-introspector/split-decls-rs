// Generated macro for try_is_word_character (function)
macro_rules! Depcratetry_is_word_character {
() => {
// Module: crate
// Provides: {"try_is_word_character"}
// Dependencies: {}
# [doc = " Returns true if and only if the given character is a Unicode word"] # [doc = " character."] # [doc = ""] # [doc = " A Unicode word character is defined by"] # [doc = " [UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties)."] # [doc = " In particular, a character"] # [doc = " is considered a word character if it is in either of the `Alphabetic` or"] # [doc = " `Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`"] # [doc = " or `Connector_Punctuation` general categories."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the `unicode-perl` feature is not enabled, then this function always"] # [doc = " returns an error."] pub fn try_is_word_character (c : char ,) -> core :: result :: Result < bool , UnicodeWordError > { unicode :: is_word_character (c) }
};
}
