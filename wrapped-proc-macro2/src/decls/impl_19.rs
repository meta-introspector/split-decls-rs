macro_rules! deps {
    () => {
        LexError!();
        TokenStream!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [doc = " Attempts to break the string into tokens and parse those tokens into a token"] # [doc = " stream."] # [doc = ""] # [doc = " May fail for a number of reasons, for example, if the string contains"] # [doc = " unbalanced delimiters or characters not existing in the language."] # [doc = ""] # [doc = " NOTE: Some errors may cause panics instead of returning `LexError`. We"] # [doc = " reserve the right to change these errors into `LexError`s later."] impl FromStr for TokenStream { type Err = LexError ; fn from_str (src : & str) -> Result < TokenStream , LexError > { match imp :: TokenStream :: from_str_checked (src) { Ok (tokens) => Ok (TokenStream :: _new (tokens)) , Err (lex) => Err (LexError { inner : lex , _marker : MARKER , }) , } } }
    };
}

impl_19!()