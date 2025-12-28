macro_rules! macro_212 {
    () => {
        make_binary_property ! { name : "Sentence_Terminal" ; short_name : "STerm" ; ident : SentenceTerminal ; data_marker : crate :: provider :: PropertyBinarySentenceTerminalV1 ; singleton : SINGLETON_PROPERTY_BINARY_SENTENCE_TERMINAL_V1 ; # [doc = " Punctuation characters that generally mark the end of sentences."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::SentenceTerminal;"] # [doc = ""] # [doc = " let sentence_terminal = CodePointSetData::new::<SentenceTerminal>();"] # [doc = ""] # [doc = " assert!(sentence_terminal.contains('.'));"] # [doc = " assert!(sentence_terminal.contains('?'));"] # [doc = " assert!(sentence_terminal.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN"] # [doc = " assert!(!sentence_terminal.contains(','));"] # [doc = " assert!(!sentence_terminal.contains('¿'));  // U+00BF INVERTED QUESTION MARK"] # [doc = " ```"] }
    };
}

macro_212!();