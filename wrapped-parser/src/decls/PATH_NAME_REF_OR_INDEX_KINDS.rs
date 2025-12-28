macro_rules! deps {
    () => {
        TokenSet!();
    };
}

macro_rules! PATH_NAME_REF_OR_INDEX_KINDS {
    () => {
        deps!();
        const PATH_NAME_REF_OR_INDEX_KINDS : TokenSet = PATH_NAME_REF_KINDS . union (TokenSet :: new (& [INT_NUMBER])) ;
    };
}

PATH_NAME_REF_OR_INDEX_KINDS!();