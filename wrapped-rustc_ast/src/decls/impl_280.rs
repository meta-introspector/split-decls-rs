macro_rules! deps {
    () => {
        MetaItemKind!();
        TokenStream!();
        TokenStreamIter!();
        ExprKind!();
        TokenTree!();
        MetaItemLit!();
        Token!();
        MetaItemInner!();
        Lit!();
        Delimiter!();
        AttrArgs!();
        DelimArgs!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl MetaItemKind { pub fn list_from_tokens (tokens : TokenStream) -> Option < ThinVec < MetaItemInner > > { let mut iter = tokens . iter () ; let mut result = ThinVec :: new () ; while iter . peek () . is_some () { let item = MetaItemInner :: from_tokens (& mut iter) ? ; result . push (item) ; match iter . next () { None | Some (TokenTree :: Token (Token { kind : token :: Comma , .. } , _)) => { } _ => return None , } } Some (result) } fn name_value_from_tokens (iter : & mut TokenStreamIter < '_ >) -> Option < MetaItemKind > { match iter . next () { Some (TokenTree :: Delimited (.. , Delimiter :: Invisible (_) , inner_tokens)) => { MetaItemKind :: name_value_from_tokens (& mut inner_tokens . iter ()) } Some (TokenTree :: Token (token , _)) => { MetaItemLit :: from_token (token) . map (MetaItemKind :: NameValue) } _ => None , } } fn from_tokens (iter : & mut TokenStreamIter < '_ >) -> Option < MetaItemKind > { match iter . peek () { Some (TokenTree :: Delimited (.. , Delimiter :: Parenthesis , inner_tokens)) => { let inner_tokens = inner_tokens . clone () ; iter . next () ; MetaItemKind :: list_from_tokens (inner_tokens) . map (MetaItemKind :: List) } Some (TokenTree :: Delimited (..)) => None , Some (TokenTree :: Token (Token { kind : token :: Eq , .. } , _)) => { iter . next () ; MetaItemKind :: name_value_from_tokens (iter) } _ => Some (MetaItemKind :: Word) , } } fn from_attr_args (args : & AttrArgs) -> Option < MetaItemKind > { match args { AttrArgs :: Empty => Some (MetaItemKind :: Word) , AttrArgs :: Delimited (DelimArgs { dspan : _ , delim : Delimiter :: Parenthesis , tokens }) => { MetaItemKind :: list_from_tokens (tokens . clone ()) . map (MetaItemKind :: List) } AttrArgs :: Delimited (..) => None , AttrArgs :: Eq { expr , .. } => match expr . kind { ExprKind :: Lit (token_lit) => { MetaItemLit :: from_token_lit (token_lit , expr . span) . ok () . map (| lit | MetaItemKind :: NameValue (lit)) } _ => None , } , } } }
    };
}

impl_280!()