macro_rules! TokenSet {
    () => {
        # [doc = " A bit-set of `SyntaxKind`s"] # [derive (Clone , Copy)] pub (crate) struct TokenSet ([u64 ; 3]) ;
    };
}

TokenSet!()