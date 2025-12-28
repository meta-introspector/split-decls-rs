macro_rules! deps {
    () => {
        MacroExprFragment2024!();
        EarlyContext!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl Expr2024 { fn check_tokens (& mut self , cx : & crate :: EarlyContext < '_ > , tokens : & TokenStream) { let mut prev_colon = false ; let mut prev_identifier = false ; let mut prev_dollar = false ; for tt in tokens . iter () { debug ! ("check_tokens: {:?} - colon {prev_dollar} - ident {prev_identifier} - colon {prev_colon}" , tt) ; match tt { TokenTree :: Token (token , _) => match token . kind { TokenKind :: Dollar => { prev_dollar = true ; continue ; } TokenKind :: Ident (..) | TokenKind :: NtIdent (..) => { if prev_colon && prev_identifier && prev_dollar { self . check_ident_token (cx , token) ; } else if prev_dollar { prev_identifier = true ; continue ; } } TokenKind :: Colon => { if prev_dollar && prev_identifier { prev_colon = true ; continue ; } } _ => { } } , TokenTree :: Delimited (.. , tts) => self . check_tokens (cx , tts) , } prev_colon = false ; prev_identifier = false ; prev_dollar = false ; } } fn check_ident_token (& mut self , cx : & crate :: EarlyContext < '_ > , token : & Token) { debug ! ("check_ident_token: {:?}" , token) ; let (sym , edition) = match token . kind { TokenKind :: Ident (sym , _) => (sym , Edition :: Edition2024) , _ => return , } ; debug ! ("token.span.edition(): {:?}" , token . span . edition ()) ; if token . span . edition () >= edition { return ; } if sym != sym :: expr { return ; } debug ! ("emitting lint") ; cx . builder . emit_span_lint (& EDITION_2024_EXPR_FRAGMENT_SPECIFIER , token . span . into () , MacroExprFragment2024 { suggestion : token . span } ,) ; } }
    };
}

impl_651!();