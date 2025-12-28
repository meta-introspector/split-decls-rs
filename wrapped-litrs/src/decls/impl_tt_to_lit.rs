macro_rules! deps {
    () => {
        TokenKind!();
        BoolLit!();
        InvalidToken!();
        Literal!();
    };
}

macro_rules! impl_tt_to_lit {
    () => {
        deps!();
        macro_rules ! impl_tt_to_lit { ([$ ($ prefix : tt) *] =>) => { impl TryFrom <$ ($ prefix) * TokenTree > for Literal < String > { type Error = InvalidToken ; fn try_from (tt : $ ($ prefix) * TokenTree) -> Result < Self , Self :: Error > { let span = tt . span () ; let res = match tt { $ ($ prefix) * TokenTree :: Group (_) => Err (TokenKind :: Group) , $ ($ prefix) * TokenTree :: Punct (_) => Err (TokenKind :: Punct) , $ ($ prefix) * TokenTree :: Ident (ref ident) if ident . to_string () == "true" => return Ok (Literal :: Bool (crate :: BoolLit :: True)) , $ ($ prefix) * TokenTree :: Ident (ref ident) if ident . to_string () == "false" => return Ok (Literal :: Bool (crate :: BoolLit :: False)) , $ ($ prefix) * TokenTree :: Ident (_) => Err (TokenKind :: Ident) , $ ($ prefix) * TokenTree :: Literal (ref lit) => Ok (lit) , } ; match res { Ok (lit) => Ok (From :: from (lit)) , Err (actual) => Err (InvalidToken { actual , expected : TokenKind :: Literal , span : span . into () , }) , } } } } }
    };
}

impl_tt_to_lit!();