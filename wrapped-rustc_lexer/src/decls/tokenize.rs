macro_rules! deps {
    () => {
        TokenKind!();
        FrontmatterAllowed!();
        Cursor!();
        Token!();
    };
}

macro_rules! tokenize {
    () => {
        deps!();
        # [doc = " Creates an iterator that produces tokens from the input string."] # [doc = ""] # [doc = " When parsing a full Rust document,"] # [doc = " first [`strip_shebang`] and then allow frontmatters with [`FrontmatterAllowed::Yes`]."] # [doc = ""] # [doc = " When tokenizing a slice of a document, be sure to disallow frontmatters with [`FrontmatterAllowed::No`]"] pub fn tokenize (input : & str , frontmatter_allowed : FrontmatterAllowed ,) -> impl Iterator < Item = Token > { let mut cursor = Cursor :: new (input , frontmatter_allowed) ; std :: iter :: from_fn (move | | { let token = cursor . advance_token () ; if token . kind != TokenKind :: Eof { Some (token) } else { None } }) }
    };
}

tokenize!();