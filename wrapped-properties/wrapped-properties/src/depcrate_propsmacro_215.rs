// Generated macro for macro_215 (macro)
macro_rules! Depcrate_propsmacro_215 {
() => {
// Module: crate::props
// Provides: {"macro_215"}
// Dependencies: {}
make_binary_property ! { name : "Emoji_Modifier" ; short_name : "EMod" ; ident : EmojiModifier ; data_marker : crate :: provider :: PropertyBinaryEmojiModifierV1 ; singleton : SINGLETON_PROPERTY_BINARY_EMOJI_MODIFIER_V1 ; # [doc = " Characters that are emoji modifiers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::EmojiModifier;"] # [doc = ""] # [doc = " let emoji_modifier = CodePointSetData::new::<EmojiModifier>();"] # [doc = ""] # [doc = " assert!(emoji_modifier.contains('\\u{1F3FD}'));  // EMOJI MODIFIER FITZPATRICK TYPE-4"] # [doc = " assert!(!emoji_modifier.contains('\\u{200C}'));  // ZERO WIDTH NON-JOINER"] # [doc = " ```"] }
};
}
