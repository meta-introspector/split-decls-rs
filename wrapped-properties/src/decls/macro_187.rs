macro_rules! macro_187 {
    () => {
        make_binary_property ! { name : "Ideographic" ; short_name : "Ideo" ; ident : Ideographic ; data_marker : crate :: provider :: PropertyBinaryIdeographicV1 ; singleton : SINGLETON_PROPERTY_BINARY_IDEOGRAPHIC_V1 ; # [doc = " Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)"] # [doc = " ideographs, or related siniform ideographs"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Ideographic;"] # [doc = ""] # [doc = " let ideographic = CodePointSetData::new::<Ideographic>();"] # [doc = ""] # [doc = " assert!(ideographic.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD"] # [doc = " assert!(!ideographic.contains('밥'));  // U+BC25 HANGUL SYLLABLE BAB"] # [doc = " ```"] }
    };
}

macro_187!();