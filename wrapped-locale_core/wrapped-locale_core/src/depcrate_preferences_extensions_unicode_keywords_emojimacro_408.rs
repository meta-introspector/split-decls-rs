// Generated macro for macro_408 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_emojimacro_408 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::emoji
// Provides: {"macro_408"}
// Dependencies: {}
enum_keyword ! (# [doc = " A Unicode Emoji Presentation Style Identifier"] # [doc = ""] # [doc = " It specifies a request for the preferred emoji"] # [doc = " presentation style. This can be used as part of the value for an HTML lang attribute,"] # [doc = " for example `<html lang=\"sr-Latn-u-em-emoji\">`."] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeEmojiPresentationStyleIdentifier)."] [Default] EmojiPresentationStyle { # [doc = " Use an emoji presentation for emoji characters if possible"] ("emoji" => Emoji) , # [doc = " Use a text presentation for emoji characters if possible"] ("text" => Text) , # [doc = " Use the default presentation for emoji characters as specified in UTR #51 Presentation Style"] [default] ("default" => Default) } , "em") ;
};
}
