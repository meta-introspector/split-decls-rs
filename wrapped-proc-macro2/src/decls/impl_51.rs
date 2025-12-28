macro_rules! deps {
    () => {
        Punct!();
        Spacing!();
        Span!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Punct { # [doc = " Creates a new `Punct` from the given character and spacing."] # [doc = ""] # [doc = " The `ch` argument must be a valid punctuation character permitted by the"] # [doc = " language, otherwise the function will panic."] # [doc = ""] # [doc = " The returned `Punct` will have the default span of `Span::call_site()`"] # [doc = " which can be further configured with the `set_span` method below."] pub fn new (ch : char , spacing : Spacing) -> Self { if let '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '^' | '|' | '~' = ch { Punct { ch , spacing , span : Span :: call_site () , } } else { panic ! ("unsupported proc macro punctuation character {:?}" , ch) ; } } # [doc = " Returns the value of this punctuation character as `char`."] pub fn as_char (& self) -> char { self . ch } # [doc = " Returns the spacing of this punctuation character, indicating whether"] # [doc = " it's immediately followed by another `Punct` in the token stream, so"] # [doc = " they can potentially be combined into a multicharacter operator"] # [doc = " (`Joint`), or it's followed by some other token or whitespace (`Alone`)"] # [doc = " so the operator has certainly ended."] pub fn spacing (& self) -> Spacing { self . spacing } # [doc = " Returns the span for this punctuation character."] pub fn span (& self) -> Span { self . span } # [doc = " Configure the span for this punctuation character."] pub fn set_span (& mut self , span : Span) { self . span = span ; } }
    };
}

impl_51!()