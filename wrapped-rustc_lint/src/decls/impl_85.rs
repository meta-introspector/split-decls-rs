macro_rules! deps {
    () => {
        BuiltinKeywordIdents!();
        UnderMacro!();
        EarlyContext!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl KeywordIdents { fn check_tokens (& mut self , cx : & EarlyContext < '_ > , tokens : & TokenStream) { let mut prev_dollar = false ; for tt in tokens . iter () { match tt { TokenTree :: Token (token , _) => { if let Some ((ident , token :: IdentIsRaw :: No)) = token . ident () { if ! prev_dollar { self . check_ident_token (cx , UnderMacro (true) , ident , "") ; } } else if let Some ((ident , token :: IdentIsRaw :: No)) = token . lifetime () { self . check_ident_token (cx , UnderMacro (true) , ident . without_first_quote () , "'" ,) ; } else if token . kind == TokenKind :: Dollar { prev_dollar = true ; continue ; } } TokenTree :: Delimited (.. , tts) => self . check_tokens (cx , tts) , } prev_dollar = false ; } } fn check_ident_token (& mut self , cx : & EarlyContext < '_ > , UnderMacro (under_macro) : UnderMacro , ident : Ident , prefix : & 'static str ,) { let (lint , edition) = match ident . name { kw :: Async | kw :: Await | kw :: Try => (KEYWORD_IDENTS_2018 , Edition :: Edition2018) , kw :: Dyn if ! under_macro => (KEYWORD_IDENTS_2018 , Edition :: Edition2018) , kw :: Gen => (KEYWORD_IDENTS_2024 , Edition :: Edition2024) , _ => return , } ; if ident . span . edition () >= edition || cx . sess () . psess . raw_identifier_spans . contains (ident . span) { return ; } cx . emit_span_lint (lint , ident . span , BuiltinKeywordIdents { kw : ident , next : edition , suggestion : ident . span , prefix } ,) ; } }
    };
}

impl_85!();