macro_rules! deps {
    () => {
        BoolLit!();
        ParseError!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl BoolLit { # [doc = " Parses the input as a bool literal. Returns an error if the input is"] # [doc = " invalid or represents a different kind of literal."] pub fn parse (s : & str) -> Result < Self , ParseError > { match s { "false" => Ok (Self :: False) , "true" => Ok (Self :: True) , _ => Err (perr (None , InvalidLiteral)) , } } # [doc = " Returns the actual Boolean value of this literal."] pub fn value (self) -> bool { match self { Self :: False => false , Self :: True => true , } } # [doc = " Returns the literal as string."] pub fn as_str (& self) -> & 'static str { match self { Self :: False => "false" , Self :: True => "true" , } } }
    };
}

impl_18!()