macro_rules! macro_213 {
    () => {
        make_binary_property ! { name : "Terminal_Punctuation" ; short_name : "Term" ; ident : TerminalPunctuation ; data_marker : crate :: provider :: PropertyBinaryTerminalPunctuationV1 ; singleton : SINGLETON_PROPERTY_BINARY_TERMINAL_PUNCTUATION_V1 ; # [doc = " Punctuation characters that generally mark the end of textual units."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::TerminalPunctuation;"] # [doc = ""] # [doc = " let terminal_punctuation = CodePointSetData::new::<TerminalPunctuation>();"] # [doc = ""] # [doc = " assert!(terminal_punctuation.contains('.'));"] # [doc = " assert!(terminal_punctuation.contains('?'));"] # [doc = " assert!(terminal_punctuation.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN"] # [doc = " assert!(terminal_punctuation.contains(','));"] # [doc = " assert!(!terminal_punctuation.contains('¿'));  // U+00BF INVERTED QUESTION MARK"] # [doc = " ```"] }
    };
}

macro_213!();