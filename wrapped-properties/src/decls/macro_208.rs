macro_rules! macro_208 {
    () => {
        make_binary_property ! { name : "Regional_Indicator" ; short_name : "RI" ; ident : RegionalIndicator ; data_marker : crate :: provider :: PropertyBinaryRegionalIndicatorV1 ; singleton : SINGLETON_PROPERTY_BINARY_REGIONAL_INDICATOR_V1 ; # [doc = " Regional indicator characters, `U+1F1E6..U+1F1FF`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::RegionalIndicator;"] # [doc = ""] # [doc = " let regional_indicator = CodePointSetData::new::<RegionalIndicator>();"] # [doc = ""] # [doc = " assert!(regional_indicator.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T"] # [doc = " assert!(!regional_indicator.contains('Ⓣ'));  // U+24C9 CIRCLED LATIN CAPITAL LETTER T"] # [doc = " assert!(!regional_indicator.contains('T'));"] # [doc = " ```"] }
    };
}

macro_208!()