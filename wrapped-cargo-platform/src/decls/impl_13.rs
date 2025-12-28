macro_rules! deps {
    () => {
        Cfg!();
        ParseError!();
        Parser!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl FromStr for Cfg { type Err = ParseError ; fn from_str (s : & str) -> Result < Cfg , Self :: Err > { let mut p = Parser :: new (s) ; let e = p . cfg () ? ; if let Some (rest) = p . rest () { return Err (ParseError :: new (p . t . orig , UnterminatedExpression (rest . to_string ()) ,)) ; } Ok (e) } }
    };
}

impl_13!();