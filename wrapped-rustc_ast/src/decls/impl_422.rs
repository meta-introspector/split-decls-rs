macro_rules! deps {
    () => {
        TokenKind!();
        Token!();
        Spacing!();
        TokenTree!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl TokenTree { # [doc = " Checks if this `TokenTree` is equal to the other, regardless of span/spacing information."] pub fn eq_unspanned (& self , other : & TokenTree) -> bool { match (self , other) { (TokenTree :: Token (token , _) , TokenTree :: Token (token2 , _)) => token . kind == token2 . kind , (TokenTree :: Delimited (.. , delim , tts) , TokenTree :: Delimited (.. , delim2 , tts2)) => { delim == delim2 && tts . len () == tts2 . len () && tts . iter () . zip (tts2 . iter ()) . all (| (a , b) | a . eq_unspanned (b)) } _ => false , } } # [doc = " Retrieves the `TokenTree`'s span."] pub fn span (& self) -> Span { match self { TokenTree :: Token (token , _) => token . span , TokenTree :: Delimited (sp , ..) => sp . entire () , } } # [doc = " Create a `TokenTree::Token` with alone spacing."] pub fn token_alone (kind : TokenKind , span : Span) -> TokenTree { TokenTree :: Token (Token :: new (kind , span) , Spacing :: Alone) } # [doc = " Create a `TokenTree::Token` with joint spacing."] pub fn token_joint (kind : TokenKind , span : Span) -> TokenTree { TokenTree :: Token (Token :: new (kind , span) , Spacing :: Joint) } # [doc = " Create a `TokenTree::Token` with joint-hidden spacing."] pub fn token_joint_hidden (kind : TokenKind , span : Span) -> TokenTree { TokenTree :: Token (Token :: new (kind , span) , Spacing :: JointHidden) } pub fn uninterpolate (& self) -> Cow < '_ , TokenTree > { match self { TokenTree :: Token (token , spacing) => match token . uninterpolate () { Cow :: Owned (token) => Cow :: Owned (TokenTree :: Token (token , * spacing)) , Cow :: Borrowed (_) => Cow :: Borrowed (self) , } , _ => Cow :: Borrowed (self) , } } }
    };
}

impl_422!()