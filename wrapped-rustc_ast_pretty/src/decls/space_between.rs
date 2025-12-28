macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! space_between {
    () => {
        deps!();
        # [doc = " Should two consecutive tokens be printed with a space between them?"] # [doc = ""] # [doc = " Note: some old proc macros parse pretty-printed output, so changes here can"] # [doc = " break old code. For example:"] # [doc = " - #63896: `#[allow(unused,` must be printed rather than `#[allow(unused ,`"] # [doc = " - #73345: `#[allow(unused)]` must be printed rather than `# [allow(unused)]`"] # [doc = ""] fn space_between (tt1 : & TokenTree , tt2 : & TokenTree) -> bool { use Delimiter :: * ; use TokenTree :: { Delimited as Del , Token as Tok } ; use token :: * ; fn is_punct (tt : & TokenTree) -> bool { matches ! (tt , TokenTree :: Token (tok , _) if tok . is_punct ()) } match (tt1 , tt2) { (Tok (Token { kind : DocComment (CommentKind :: Line , ..) , .. } , _) , _) => false , (Tok (Token { kind : Dot , .. } , _) , tt2) if ! is_punct (tt2) => false , (Tok (Token { kind : Dollar , .. } , _) , Tok (Token { kind : Ident (..) , .. } , _)) => false , (tt1 , Tok (Token { kind : Comma | Semi | Dot , .. } , _)) if ! is_punct (tt1) => false , (Tok (Token { kind : Ident (sym , is_raw) , span } , _) , Tok (Token { kind : Bang , .. } , _)) if ! Ident :: new (* sym , * span) . is_reserved () || matches ! (is_raw , IdentIsRaw :: Yes) => { false } (Tok (Token { kind : Ident (sym , is_raw) , span } , _) , Del (_ , _ , Parenthesis , _)) if ! Ident :: new (* sym , * span) . is_reserved () || * sym == kw :: Fn || * sym == kw :: SelfUpper || * sym == kw :: Pub || matches ! (is_raw , IdentIsRaw :: Yes) => { false } (Tok (Token { kind : Pound , .. } , _) , Del (_ , _ , Bracket , _)) => false , _ => true , } }
    };
}

space_between!();