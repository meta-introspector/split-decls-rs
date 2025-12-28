macro_rules! macro_180 {
    () => {
        make_binary_property ! { name : "Grapheme_Extend" ; short_name : "Gr_Ext" ; ident : GraphemeExtend ; data_marker : crate :: provider :: PropertyBinaryGraphemeExtendV1 ; singleton : SINGLETON_PROPERTY_BINARY_GRAPHEME_EXTEND_V1 ; # [doc = " Property used to define \"Grapheme extender\"."] # [doc = ""] # [doc = " See D59 in Chapter 3, Conformance in the"] # [doc = " Unicode Standard."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::GraphemeExtend;"] # [doc = ""] # [doc = " let grapheme_extend = CodePointSetData::new::<GraphemeExtend>();"] # [doc = ""] # [doc = " assert!(!grapheme_extend.contains('ക'));  // U+0D15 MALAYALAM LETTER KA"] # [doc = " assert!(!grapheme_extend.contains('\\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I"] # [doc = " assert!(grapheme_extend.contains('\\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA"] # [doc = " ```"] }
    };
}

macro_180!();