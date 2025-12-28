macro_rules! deps {
    () => {
        Literal!();
        TokenKind!();
        InvalidToken!();
    };
}

macro_rules! impl_for_specific_lit {
    () => {
        deps!();
        macro_rules ! impl_for_specific_lit { ([$ ($ prefix : tt) *] => $ ty : ty , $ variant : ident , $ kind : ident) => { impl TryFrom <$ ($ prefix) * Literal > for $ ty { type Error = InvalidToken ; fn try_from (src : $ ($ prefix) * Literal) -> Result < Self , Self :: Error > { let span = src . span () ; let lit : Literal < String > = src . into () ; match lit { Literal ::$ variant (s) => Ok (s) , other => Err (InvalidToken { expected : TokenKind ::$ kind , actual : kind_of (& other) , span : span . into () , }) , } } } impl TryFrom <$ ($ prefix) * TokenTree > for $ ty { type Error = InvalidToken ; fn try_from (tt : $ ($ prefix) * TokenTree) -> Result < Self , Self :: Error > { let span = tt . span () ; let res = match tt { $ ($ prefix) * TokenTree :: Group (_) => Err (TokenKind :: Group) , $ ($ prefix) * TokenTree :: Punct (_) => Err (TokenKind :: Punct) , $ ($ prefix) * TokenTree :: Ident (_) => Err (TokenKind :: Ident) , $ ($ prefix) * TokenTree :: Literal (ref lit) => Ok (lit) , } ; match res { Ok (lit) => <$ ty >:: try_from (lit) , Err (actual) => Err (InvalidToken { actual , expected : TokenKind ::$ kind , span : span . into () , }) , } } } } ; }
    };
}

impl_for_specific_lit!()