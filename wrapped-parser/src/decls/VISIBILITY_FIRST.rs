macro_rules! deps {
    () => {
        TokenSet!();
    };
}

macro_rules! VISIBILITY_FIRST {
    () => {
        deps!();
        const VISIBILITY_FIRST : TokenSet = TokenSet :: new (& [T ! [pub]]) ;
    };
}

VISIBILITY_FIRST!();