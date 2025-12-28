macro_rules! macro_171 {
    () => {
        make_binary_property ! { name : "Emoji_Modifier_Base" ; short_name : "EBase" ; ident : EmojiModifierBase ; data_marker : crate :: provider :: PropertyBinaryEmojiModifierBaseV1 ; singleton : SINGLETON_PROPERTY_BINARY_EMOJI_MODIFIER_BASE_V1 ; # [doc = " Characters that can serve as a base for emoji modifiers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::EmojiModifierBase;"] # [doc = ""] # [doc = " let emoji_modifier_base = CodePointSetData::new::<EmojiModifierBase>();"] # [doc = ""] # [doc = " assert!(emoji_modifier_base.contains('✊'));  // U+270A RAISED FIST"] # [doc = " assert!(!emoji_modifier_base.contains('⛰'));  // U+26F0 MOUNTAIN"] # [doc = " ```"] }
    };
}

macro_171!();