macro_rules! deps {
    () => {
        TokenKind!();
        Literal!();
        InvalidToken!();
        BoolLit!();
    };
}

macro_rules! impl_from_tt_for_bool {
    () => {
        deps!();
        macro_rules ! impl_from_tt_for_bool { ([$ ($ prefix : tt) *] =>) => { impl TryFrom <$ ($ prefix) * TokenTree > for crate :: BoolLit { type Error = InvalidToken ; fn try_from (tt : $ ($ prefix) * TokenTree) -> Result < Self , Self :: Error > { let span = tt . span () ; let actual = match tt { $ ($ prefix) * TokenTree :: Ident (ref ident) if ident . to_string () == "true" => return Ok (crate :: BoolLit :: True) , $ ($ prefix) * TokenTree :: Ident (ref ident) if ident . to_string () == "false" => return Ok (crate :: BoolLit :: False) , $ ($ prefix) * TokenTree :: Group (_) => TokenKind :: Group , $ ($ prefix) * TokenTree :: Punct (_) => TokenKind :: Punct , $ ($ prefix) * TokenTree :: Ident (_) => TokenKind :: Ident , $ ($ prefix) * TokenTree :: Literal (ref lit) => kind_of (& Literal :: from (lit)) , } ; Err (InvalidToken { actual , expected : TokenKind :: BoolLit , span : span . into () , }) } } } ; }
    };
}

impl_from_tt_for_bool!()