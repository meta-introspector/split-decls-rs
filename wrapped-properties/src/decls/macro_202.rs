macro_rules! macro_202 {
    () => {
        make_binary_property ! { name : "Pattern_Syntax" ; short_name : "Pat_Syn" ; ident : PatternSyntax ; data_marker : crate :: provider :: PropertyBinaryPatternSyntaxV1 ; singleton : SINGLETON_PROPERTY_BINARY_PATTERN_SYNTAX_V1 ; # [doc = " Characters used as syntax in patterns (such as regular expressions)."] # [doc = ""] # [doc = " See [`Unicode"] # [doc = " Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more"] # [doc = " details."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::PatternSyntax;"] # [doc = ""] # [doc = " let pattern_syntax = CodePointSetData::new::<PatternSyntax>();"] # [doc = ""] # [doc = " assert!(pattern_syntax.contains('{'));"] # [doc = " assert!(pattern_syntax.contains('⇒'));  // U+21D2 RIGHTWARDS DOUBLE ARROW"] # [doc = " assert!(!pattern_syntax.contains('0'));"] # [doc = " ```"] }
    };
}

macro_202!();