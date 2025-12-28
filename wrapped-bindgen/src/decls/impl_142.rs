macro_rules! deps {
    () => {
        Delimiter!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl Delimiter { # [doc = " The opening delimiter"] pub fn open (self) -> char { match self { Self :: Bracket => '[' , Self :: Brace => '{' , Self :: Parenthesis => '(' , } } # [doc = " The closing delimiter"] pub fn close (self) -> char { match self { Self :: Bracket => ']' , Self :: Brace => '}' , Self :: Parenthesis => ')' , } } }
    };
}

impl_142!()