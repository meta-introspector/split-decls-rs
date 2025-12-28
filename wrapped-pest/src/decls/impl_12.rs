macro_rules! deps {
    () => {
        IsWhitespaceFn!();
        ParsingToken!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl ParsingToken { pub fn is_whitespace (& self , is_whitespace : & IsWhitespaceFn) -> bool { match self { ParsingToken :: Sensitive { token } => is_whitespace (token . clone ()) , ParsingToken :: Insensitive { token } => is_whitespace (token . clone ()) , ParsingToken :: Range { .. } => false , ParsingToken :: BuiltInRule => false , } } }
    };
}

impl_12!()