macro_rules! deps {
    () => {
        ParsingToken!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Display for ParsingToken { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { match self { ParsingToken :: Sensitive { token } => write ! (f , "{token}") , ParsingToken :: Insensitive { token } => write ! (f , "{}" , token . to_uppercase ()) , ParsingToken :: Range { start , end } => write ! (f , "{start}..{end}") , ParsingToken :: BuiltInRule => write ! (f , "BUILTIN_RULE") , } } }
    };
}

impl_100!()