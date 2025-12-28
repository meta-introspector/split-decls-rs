macro_rules! macro_203 {
    () => {
        make_binary_property ! { name : "Pattern_White_Space" ; short_name : "Pat_WS" ; ident : PatternWhiteSpace ; data_marker : crate :: provider :: PropertyBinaryPatternWhiteSpaceV1 ; singleton : SINGLETON_PROPERTY_BINARY_PATTERN_WHITE_SPACE_V1 ; # [doc = " Characters used as whitespace in patterns (such as regular expressions)."] # [doc = ""] # [doc = " See"] # [doc = " [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for"] # [doc = " more details."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::PatternWhiteSpace;"] # [doc = ""] # [doc = " let pattern_white_space = CodePointSetData::new::<PatternWhiteSpace>();"] # [doc = ""] # [doc = " assert!(pattern_white_space.contains(' '));"] # [doc = " assert!(pattern_white_space.contains('\\u{2029}'));  // PARAGRAPH SEPARATOR"] # [doc = " assert!(pattern_white_space.contains('\\u{000A}'));  // NEW LINE"] # [doc = " assert!(!pattern_white_space.contains('\\u{00A0}'));  // NO-BREAK SPACE"] # [doc = " ```"] }
    };
}

macro_203!();