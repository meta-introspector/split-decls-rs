macro_rules! macro_216 {
    () => {
        make_binary_property ! { name : "Variation_Selector" ; short_name : "VS" ; ident : VariationSelector ; data_marker : crate :: provider :: PropertyBinaryVariationSelectorV1 ; singleton : SINGLETON_PROPERTY_BINARY_VARIATION_SELECTOR_V1 ; # [doc = " Characters that are Variation Selectors."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::VariationSelector;"] # [doc = ""] # [doc = " let variation_selector = CodePointSetData::new::<VariationSelector>();"] # [doc = ""] # [doc = " assert!(variation_selector.contains('\\u{180D}'));  // MONGOLIAN FREE VARIATION SELECTOR THREE"] # [doc = " assert!(!variation_selector.contains('\\u{303E}'));  // IDEOGRAPHIC VARIATION INDICATOR"] # [doc = " assert!(variation_selector.contains('\\u{FE0F}'));  // VARIATION SELECTOR-16"] # [doc = " assert!(!variation_selector.contains('\\u{FE10}'));  // PRESENTATION FORM FOR VERTICAL COMMA"] # [doc = " assert!(variation_selector.contains('\\u{E01EF}'));  // VARIATION SELECTOR-256"] # [doc = " ```"] }
    };
}

macro_216!();