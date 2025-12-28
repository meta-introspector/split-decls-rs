macro_rules! deps {
    () => {
        TokenSet!();
    };
}

macro_rules! PATH_NAME_REF_KINDS {
    () => {
        deps!();
        const PATH_NAME_REF_KINDS : TokenSet = TokenSet :: new (& [IDENT , T ! [self] , T ! [super] , T ! [crate] , T ! [Self]]) ;
    };
}

PATH_NAME_REF_KINDS!();