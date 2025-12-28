macro_rules! deps {
    () => {
        Ident!();
        Token!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a > Token < 'a > { fn classify (& self) -> & 'static str { match * self { Token :: LeftParen => "`(`" , Token :: RightParen => "`)`" , Token :: Ident (..) => "an identifier" , Token :: Comma => "`,`" , Token :: Equals => "`=`" , Token :: String (..) => "a string" , } } }
    };
}

impl_24!();