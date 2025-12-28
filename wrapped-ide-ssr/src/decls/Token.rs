macro_rules! Token {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub (crate) struct Token { kind : SyntaxKind , pub (crate) text : SmolStr , }
    };
}

Token!()