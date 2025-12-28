macro_rules! deps {
    () => {
        TokenTree!();
        AttrTokenTree!();
        Token!();
        AttrTokenStream!();
        TokenStream!();
        AttrsTarget!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl AttrTokenStream { pub fn new (tokens : Vec < AttrTokenTree >) -> AttrTokenStream { AttrTokenStream (Arc :: new (tokens)) } # [doc = " Converts this `AttrTokenStream` to a plain `Vec<TokenTree>`. During"] # [doc = " conversion, any `AttrTokenTree::AttrsTarget` gets \"flattened\" back to a"] # [doc = " `TokenStream`, as described in the comment on"] # [doc = " `attrs_and_tokens_to_token_trees`."] pub fn to_token_trees (& self) -> Vec < TokenTree > { let mut res = Vec :: with_capacity (self . 0 . len ()) ; for tree in self . 0 . iter () { match tree { AttrTokenTree :: Token (inner , spacing) => { res . push (TokenTree :: Token (inner . clone () , * spacing)) ; } AttrTokenTree :: Delimited (span , spacing , delim , stream) => { res . push (TokenTree :: Delimited (* span , * spacing , * delim , TokenStream :: new (stream . to_token_trees ()) ,)) } AttrTokenTree :: AttrsTarget (target) => { attrs_and_tokens_to_token_trees (& target . attrs , & target . tokens , & mut res) ; } } } res } }
    };
}

impl_440!();