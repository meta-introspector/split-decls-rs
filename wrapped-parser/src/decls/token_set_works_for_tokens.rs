macro_rules! deps {
    () => {
        TokenSet!();
    };
}

macro_rules! token_set_works_for_tokens {
    () => {
        deps!();
        # [test] fn token_set_works_for_tokens () { use crate :: SyntaxKind :: * ; let ts = TokenSet :: new (& [EOF , SHEBANG]) ; assert ! (ts . contains (EOF)) ; assert ! (ts . contains (SHEBANG)) ; assert ! (! ts . contains (PLUS)) ; }
    };
}

token_set_works_for_tokens!();