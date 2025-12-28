macro_rules! macro_179 {
    () => {
        make_binary_property ! { name : "Grapheme_Base" ; short_name : "Gr_Base" ; ident : GraphemeBase ; data_marker : crate :: provider :: PropertyBinaryGraphemeBaseV1 ; singleton : SINGLETON_PROPERTY_BINARY_GRAPHEME_BASE_V1 ; # [doc = " Property used together with the definition of Standard Korean Syllable Block to define"] # [doc = " \"Grapheme base\"."] # [doc = ""] # [doc = " See D58 in Chapter 3, Conformance in the Unicode Standard."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::GraphemeBase;"] # [doc = ""] # [doc = " let grapheme_base = CodePointSetData::new::<GraphemeBase>();"] # [doc = ""] # [doc = " assert!(grapheme_base.contains('ക'));  // U+0D15 MALAYALAM LETTER KA"] # [doc = " assert!(grapheme_base.contains('\\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I"] # [doc = " assert!(!grapheme_base.contains('\\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA"] # [doc = " ```"] }
    };
}

macro_179!();