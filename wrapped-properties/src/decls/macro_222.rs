macro_rules! macro_222 {
    () => {
        make_emoji_set ! { ident : BasicEmoji ; data_marker : crate :: provider :: PropertyBinaryBasicEmojiV1 ; singleton : SINGLETON_PROPERTY_BINARY_BASIC_EMOJI_V1 ; # [doc = " Characters and character sequences intended for general-purpose, independent, direct input."] # [doc = ""] # [doc = " See [`Unicode Technical Standard #51`](https://unicode.org/reports/tr51/) for more"] # [doc = " details."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::EmojiSetData;"] # [doc = " use icu::properties::props::BasicEmoji;"] # [doc = ""] # [doc = " let basic_emoji = EmojiSetData::new::<BasicEmoji>();"] # [doc = ""] # [doc = " assert!(!basic_emoji.contains('\\u{0020}'));"] # [doc = " assert!(!basic_emoji.contains('\\n'));"] # [doc = " assert!(basic_emoji.contains('🦃')); // U+1F983 TURKEY"] # [doc = " assert!(basic_emoji.contains_str(\"\\u{1F983}\"));"] # [doc = " assert!(basic_emoji.contains_str(\"\\u{1F6E4}\\u{FE0F}\")); // railway track"] # [doc = " assert!(!basic_emoji.contains_str(\"\\u{0033}\\u{FE0F}\\u{20E3}\"));  // Emoji_Keycap_Sequence, keycap 3"] # [doc = " ```"] }
    };
}

macro_222!()