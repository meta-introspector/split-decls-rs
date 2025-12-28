macro_rules! macro_175 {
    () => {
        make_binary_property ! { name : "Emoji_Presentation" ; short_name : "EPres" ; ident : EmojiPresentation ; data_marker : crate :: provider :: PropertyBinaryEmojiPresentationV1 ; singleton : SINGLETON_PROPERTY_BINARY_EMOJI_PRESENTATION_V1 ; # [doc = " Characters that have emoji presentation by default."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::EmojiPresentation;"] # [doc = ""] # [doc = " let emoji_presentation = CodePointSetData::new::<EmojiPresentation>();"] # [doc = ""] # [doc = " assert!(emoji_presentation.contains('🦬')); // U+1F9AC BISON"] # [doc = " assert!(!emoji_presentation.contains('♻'));  // U+267B BLACK UNIVERSAL RECYCLING SYMBOL"] # [doc = " ```"] }
    };
}

macro_175!();