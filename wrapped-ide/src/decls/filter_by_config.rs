macro_rules! deps {
    () => {
        HighlightConfig!();
    };
}

macro_rules! filter_by_config {
    () => {
        deps!();
        fn filter_by_config (highlight : & mut Highlight , config : & HighlightConfig < '_ >) -> bool { match & mut highlight . tag { HlTag :: StringLiteral if ! config . strings => return false , HlTag :: Comment if ! config . comments => return false , tag @ HlTag :: Punctuation (HlPunct :: MacroBang) => { if ! config . macro_bang { * tag = HlTag :: Symbol (SymbolKind :: Macro) ; } else if ! config . specialize_punctuation { * tag = HlTag :: Punctuation (HlPunct :: Other) ; } } HlTag :: Punctuation (_) if ! config . punctuation && highlight . mods . is_empty () => return false , tag @ HlTag :: Punctuation (_) if ! config . specialize_punctuation => { * tag = HlTag :: Punctuation (HlPunct :: Other) ; } HlTag :: Operator (_) if ! config . operator && highlight . mods . is_empty () => return false , tag @ HlTag :: Operator (_) if ! config . specialize_operator => { * tag = HlTag :: Operator (HlOperator :: Other) ; } _ => () , } true }
    };
}

filter_by_config!();