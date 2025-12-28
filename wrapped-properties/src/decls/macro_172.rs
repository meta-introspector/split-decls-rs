macro_rules! macro_172 {
    () => {
        make_binary_property ! { name : "Emoji_Component" ; short_name : "EComp" ; ident : EmojiComponent ; data_marker : crate :: provider :: PropertyBinaryEmojiComponentV1 ; singleton : SINGLETON_PROPERTY_BINARY_EMOJI_COMPONENT_V1 ; # [doc = " Characters used in emoji sequences that normally do not appear on emoji keyboards as"] # [doc = " separate choices, such as base characters for emoji keycaps."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::EmojiComponent;"] # [doc = ""] # [doc = " let emoji_component = CodePointSetData::new::<EmojiComponent>();"] # [doc = ""] # [doc = " assert!(emoji_component.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T"] # [doc = " assert!(emoji_component.contains('\\u{20E3}'));  // COMBINING ENCLOSING KEYCAP"] # [doc = " assert!(emoji_component.contains('7'));"] # [doc = " assert!(!emoji_component.contains('T'));"] # [doc = " ```"] }
    };
}

macro_172!()