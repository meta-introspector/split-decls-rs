macro_rules! deps {
    () => {
        TokenSet!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl TokenSet { pub (crate) const EMPTY : TokenSet = TokenSet ([0 ; 3]) ; pub (crate) const fn new (kinds : & [SyntaxKind]) -> TokenSet { let mut res = [0 ; 3] ; let mut i = 0 ; while i < kinds . len () { let discriminant = kinds [i] as usize ; debug_assert ! (discriminant <= LAST_TOKEN_KIND_DISCRIMINANT , "Expected a token `SyntaxKind`") ; let idx = discriminant / 64 ; res [idx] |= 1 << (discriminant % 64) ; i += 1 ; } TokenSet (res) } pub (crate) const fn union (self , other : TokenSet) -> TokenSet { TokenSet ([self . 0 [0] | other . 0 [0] , self . 0 [1] | other . 0 [1] , self . 0 [2] | other . 0 [2]]) } pub (crate) const fn contains (& self , kind : SyntaxKind) -> bool { let discriminant = kind as usize ; debug_assert ! (discriminant <= LAST_TOKEN_KIND_DISCRIMINANT , "Expected a token `SyntaxKind`") ; let idx = discriminant / 64 ; let mask = 1 << (discriminant % 64) ; self . 0 [idx] & mask != 0 } }
    };
}

impl_92!()