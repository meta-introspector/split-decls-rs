macro_rules! macro_214 {
    () => {
        make_binary_property ! { name : "Unified_Ideograph" ; short_name : "UIdeo" ; ident : UnifiedIdeograph ; data_marker : crate :: provider :: PropertyBinaryUnifiedIdeographV1 ; singleton : SINGLETON_PROPERTY_BINARY_UNIFIED_IDEOGRAPH_V1 ; # [doc = " A property which specifies the exact set of Unified CJK Ideographs in the standard."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::UnifiedIdeograph;"] # [doc = ""] # [doc = " let unified_ideograph = CodePointSetData::new::<UnifiedIdeograph>();"] # [doc = ""] # [doc = " assert!(unified_ideograph.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD"] # [doc = " assert!(unified_ideograph.contains('木'));  // U+6728 CJK UNIFIED IDEOGRAPH-6728"] # [doc = " assert!(!unified_ideograph.contains('𛅸'));  // U+1B178 NUSHU CHARACTER-1B178"] # [doc = " ```"] }
    };
}

macro_214!();