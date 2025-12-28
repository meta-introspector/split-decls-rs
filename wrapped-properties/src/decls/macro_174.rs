macro_rules! macro_174 {
    () => {
        make_binary_property ! { name : "Emoji" ; short_name : "Emoji" ; ident : Emoji ; data_marker : crate :: provider :: PropertyBinaryEmojiV1 ; singleton : SINGLETON_PROPERTY_BINARY_EMOJI_V1 ; # [doc = " Characters that are emoji."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Emoji;"] # [doc = ""] # [doc = " let emoji = CodePointSetData::new::<Emoji>();"] # [doc = ""] # [doc = " assert!(emoji.contains('🔥'));  // U+1F525 FIRE"] # [doc = " assert!(!emoji.contains('V'));"] # [doc = " ```"] }
    };
}

macro_174!();